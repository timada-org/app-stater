use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct StarterConfig {
    #[serde(default = "default_api_addr")]
    pub api_addr: String,

    #[serde(default = "default_app_addr")]
    pub app_addr: String,
}

fn default_api_addr() -> String {
    "0.0.0.0:4000".to_string()
}

fn default_app_addr() -> String {
    "0.0.0.0:3000".to_string()
}

impl StarterConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("STARTER_CONFIG_PATH").unwrap_or("./starter.toml".to_owned());

        Config::builder()
            .add_source(
                File::with_name(&config_path).required(env::var("STARTER_CONFIG_PATH").is_ok()),
            )
            .add_source(Environment::with_prefix("starter"))
            .build()?
            .try_deserialize()
    }
}
