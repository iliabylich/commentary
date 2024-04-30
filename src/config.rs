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

#[cfg(debug_assertions)]
const CONFIG_PATH: &str = "config.json";

#[cfg(not(debug_assertions))]
const CONFIG_PATH: &str = "/etc/commentary.json";

const DEFAULT_CONFIG: &str = include_str!("../config.example.json");

impl Config {
    pub(crate) fn load() -> Result<()> {
        if !std::path::Path::new(CONFIG_PATH).exists() {
            std::fs::write(CONFIG_PATH, DEFAULT_CONFIG).unwrap()
        }

        let file = std::fs::File::open(CONFIG_PATH).context("Failed to open config file")?;
        let config: Config =
            serde_json::from_reader::<_, Config>(&file).context("Failed to parse config file")?;
        CONFIG.set(config).context("Config has already been loaded")
    }

    pub(crate) fn global() -> Result<&'static Config> {
        CONFIG.get().context("Config is not loaded")
    }
}
