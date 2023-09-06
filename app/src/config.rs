use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use starter_core::AppConfig;
use std::env;
#[derive(Debug, Deserialize)]
pub struct StarterConfig {
    #[serde(default)]
    pub app: AppConfig,
}

impl StarterConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("STARTER_CONFIG_PATH");
        let config_path_required = config_path.is_ok();

        Config::builder()
            .add_source(
                File::with_name(&config_path.unwrap_or_default()).required(config_path_required),
            )
            .add_source(Environment::with_prefix("starter"))
            .build()?
            .try_deserialize()
    }
}
