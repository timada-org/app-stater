mod page;

use leptos::use_context;
pub use page::*;

#[derive(Debug, Clone)]
pub struct AppContext {
    pub base_url: Option<String>,
}

impl AppContext {
    pub fn create_url(&self, uri: impl Into<String>) -> String {
        let uri = uri.into();
        self.base_url
            .as_ref()
            .map(|base_url| format!("{base_url}{}", uri))
            .unwrap_or(uri)
    }

    pub fn create_static_url(&self, uri: impl Into<String>) -> String {
        self.create_url(format!("/static/{}", uri.into()))
    }
}

pub fn use_app() -> AppContext {
    use_context::<AppContext>().expect("AppContext not configured correctly")
}
