use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::{comment::Comment, state::AppState};

pub(crate) struct Mailer;

impl Mailer {
    pub(crate) async fn spawn(state: AppState) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));
        tokio::task::spawn(async move {
            loop {
                interval.tick().await;
                Self::tick(state.clone()).await;
            }
        })
        .await
        .expect("Failed to spawn mailer task");
    }

    async fn tick(state: AppState) {
        let new_comments = state.database.get_new_comments().await;
        if new_comments.is_empty() {
            return;
        }
        state.mailer.send_new_comments(&new_comments).await;
    }
}

#[derive(Clone)]
pub(crate) struct Gmail {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}
impl Gmail {
    pub(crate) fn from_global_config() -> Self {
        let config = crate::config::Config::global();

        let credentials = Credentials::new(
            config.gmail_email.to_owned(),
            config.gmail_password.to_owned(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .expect("Failed to create mailer")
            .credentials(credentials)
            .build();
        Self { mailer }
    }

    async fn send_message(&self, message: Message) {
        match self.mailer.send(message).await {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }

    async fn send_new_comments(&self, comments: &[Comment]) {
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
                    .expect("Invalid email"),
            )
            .to("Ilya Bylich <ibylich@gmail.com>"
                .parse()
                .expect("Invalid email"))
            .subject("New comment")
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .expect("Failed to build email message");

        self.send_message(message).await;
    }
}
