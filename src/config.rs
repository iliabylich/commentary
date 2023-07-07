use tokio::sync::OnceCell;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Config {
    pub(crate) database_path: String,
    pub(crate) listen_on: u16,
}

static CONFIG: OnceCell<Config> = OnceCell::const_new();

impl Config {
    pub(crate) fn load() {
        let path = if cfg!(debug_assertions) {
            std::env::var("COMMENTARY_CONFIG_PATH")
                .expect("No COMMENTARY_CONFIG_PATH environment variable set")
        } else {
            String::from("/etc/commentary.json")
        };

        let config_file = std::fs::read_to_string(&path).expect("Failed to read config file");
        let config: Config =
            serde_json::from_str(&config_file).expect("Failed to parse config file");
        CONFIG.set(config).expect("Config has already been loaded");
    }

    pub(crate) fn global() -> &'static Config {
        CONFIG.get().expect("Config is not loaded")
    }
}
