use leptos::use_context;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub config: AppConfig,
}

impl AppContext {
    pub fn create_url(&self, uri: impl Into<String>) -> String {
        let uri = uri.into();
        self.config
            .base_url
            .as_ref()
            .map(|base_url| format!("{base_url}{}", uri))
            .unwrap_or(uri)
    }

    pub fn create_static_url(&self, uri: impl Into<String>) -> String {
        self.create_url(format!("/static/{}", uri.into()))
    }
}

pub fn use_app() -> AppContext {
    use_context().expect("AppContext not configured correctly")
}
