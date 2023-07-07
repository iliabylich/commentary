mod comment;
mod config;
mod database;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{comment::Comment, config::Config, database::Database, state::AppState};

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Comment::create_table(&db).await;

    let state = AppState::new(db);

    Ok(())
}
