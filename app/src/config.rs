use config::{ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub addr: String,
    pub base_url: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:3000".to_string(),
            base_url: Some("/starter".to_owned()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub app: AppConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("STARTER_CONFIG_PATH");
        let config_path_required = config_path.is_ok();

        config::Config::builder()
            .add_source(
                File::with_name(&config_path.unwrap_or_default()).required(config_path_required),
            )
            .add_source(Environment::with_prefix("starter"))
            .build()?
            .try_deserialize()
    }
}
