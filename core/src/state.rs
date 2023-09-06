use axum::response::{IntoResponse, Html};
use starter_components::AppContext;
use leptos::*;

use crate::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub fn to_context(&self) -> AppContext {
        AppContext {
            base_url: self.config.base_url.to_owned(),
        }
    }

    pub fn render_to_string<F, N>(&self, f: F) -> impl IntoResponse
    where
        F: FnOnce() -> N + 'static,
        N: IntoView,
    {
        let ctx = self.to_context();

        let html = ssr::render_to_string(move || {
            provide_context(ctx);

            f()
        });

        Html(html.to_string())
    }
}
