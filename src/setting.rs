use config::Config;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub repeat: Option<u32>,
    #[serde(default, rename = "rpc")]
    pub rpcs: Vec<SettingClient>,
}

// This represents an RPC in the settings
#[derive(Debug, serde::Deserialize)]
pub struct SettingClient {
    pub label: String,
    pub url: String,
}

impl Settings {
    pub fn new() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name(
                std::env::var("SETTINGS")
                    .unwrap_or("config.toml".to_string())
                    .as_str(),
            ))
            .build()
            .unwrap();
        settings
            .try_deserialize::<Settings>()
            .expect("couldn't load settings file")
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
