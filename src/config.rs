use anyhow::{Context, Result};
use tokio::sync::OnceCell;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) database_path: String,
    pub(crate) listen_on: u16,
    pub(crate) gmail_email: String,
    pub(crate) gmail_password: String,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

impl Config {
    pub(crate) fn load() -> Result<()> {
        let path = if cfg!(debug_assertions) {
            std::env::var("COMMENTARY_CONFIG_PATH")
                .context("No COMMENTARY_CONFIG_PATH environment variable set")?
        } else {
            String::from("/etc/commentary.json")
        };

        let config_file = std::fs::read_to_string(path).context("Failed to read config file")?;
        let config: Config =
            serde_json::from_str(&config_file).context("Failed to parse config file")?;
        CONFIG.set(config).context("Config has already been loaded")
    }

    pub(crate) fn global() -> Result<&'static Config> {
        CONFIG.get().context("Config is not loaded")
    }
}
