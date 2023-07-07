mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;

    Config::load();

    Ok(())
}
