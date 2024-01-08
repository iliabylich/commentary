mod comment;
mod config;
mod database;
mod mailer;
mod resource;
mod state;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        comment::Comment,
        config::Config,
        database::Database,
        mailer::{Gmail, Mailer},
        state::AppState,
        web::Web,
    };

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Comment::create_table(&db).await;

    let mailer = Gmail::from_global_config();

    let state = AppState::new(db, mailer);

    tokio::join!(Web::spawn(state.clone()), Mailer::spawn(state.clone()),);

    Ok(())
}
