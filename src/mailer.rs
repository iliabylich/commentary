use anyhow::{Context, Result};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{comment::Comment, config::Config, state::AppState};

pub(crate) struct Mailer;

impl Mailer {
    pub(crate) async fn spawn(state: AppState) -> Result<()> {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));
        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                match Self::tick(state.clone()).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Failed to tick mailer: {:?}", err);
                    }
                }
            }
        })
        .await
        .context("Failed to spawn mailer task")
    }

    async fn tick(state: AppState) -> Result<()> {
        let new_comments = state.database.get_new_comments().await?;
        if new_comments.is_empty() {
            return Ok(());
        }
        state.mailer.send_new_comments(&new_comments).await
    }
}

#[derive(Clone)]
pub(crate) struct Gmail {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}
impl Gmail {
    pub(crate) fn from_global_config() -> Result<Self> {
        let config = Config::global()?;
        let credentials = Credentials::from((&config.gmail_email, &config.gmail_password));

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .context("Failed to create mailer")?
            .credentials(credentials)
            .build();
        Ok(Self { mailer })
    }

    async fn send_message(&self, message: Message) {
        match self.mailer.send(message).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    async fn send_new_comments(&self, comments: &[Comment]) -> Result<()> {
        let body = comments
            .iter()
            .map(|comment| {
                format!(
                    "{} (on {}): {}",
                    comment.author, comment.post_id, comment.body
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        let message = Message::builder()
            .from(
                "Commentary app <ibylich@gmail.com>"
                    .parse()
                    .context("Invalid email")?,
            )
            .to("Ilya Bylich <ibylich@gmail.com>"
                .parse()
                .context("Invalid email")?)
            .subject("New comment")
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .context("Failed to build email message")?;

        self.send_message(message).await;

        Ok(())
    }
}
