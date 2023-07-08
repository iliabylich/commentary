mod comment;
mod config;
mod database;
mod resource;
mod state;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        comment::Comment, config::Config, database::Database, resource::Resources, state::AppState,
        web::Web,
    };

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Comment::create_table(&db).await;

    let resources = Resources::new();

    let state = AppState::new(db, resources);

    Web::spawn(state).await;

    Ok(())
}
