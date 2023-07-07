mod config;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{config::Config, database::Database};

    Config::load();

    let db = Database::new().await;

    Ok(())
}
