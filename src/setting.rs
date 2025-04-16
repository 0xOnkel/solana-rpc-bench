use config::Config;

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub rpcs: Vec<String>,
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
