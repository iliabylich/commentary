mod comment;
mod config;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{comment::Comment, config::Config, database::Database};

    Config::load();
    println!("Running with config {:?}", Config::global());

    let db = Database::new().await;
    Comment::create_table(&db).await;

    Ok(())
}
