use serde::Deserialize;

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
