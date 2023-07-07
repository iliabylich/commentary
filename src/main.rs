mod comment;
mod config;
mod database;
mod state;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{comment::Comment, config::Config, database::Database, state::AppState, web::Web};

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Comment::create_table(&db).await;

    let state = AppState::new(db);

    Web::spawn(state).await;

    Ok(())
}
