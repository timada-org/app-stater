use config::{ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub addr: String,
    pub base_url: Option<String>,
    pub jwks_url: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:3000".to_string(),
            base_url: Some("/starter".to_owned()),
            jwks_url: Some(
                "http://127.0.0.1:4456/.well-known/jwks.json".to_owned(),
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub app: AppConfig,

    #[serde(default = "default_dsn")]
    pub dsn: String,
}

fn default_dsn() -> String {
    "cockroach://starter@127.0.0.1:26257/starter?sslmode=disable".to_owned()
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
