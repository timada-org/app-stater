use axum::response::{Html, IntoResponse};
use evento::Producer;
use evento_store::PgEngine;
use i18n_embed::{fluent::FluentLanguageLoader, LanguageLoader};
use leptos::*;
use serde::Deserialize;
use unic_langid::LanguageIdentifier;

use crate::{
    config::AppConfig,
    context::AppContext,
    i18n::{LANGUAGES, LANGUAGE_LOADER},
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub evento: Producer<PgEngine>,
}

impl AppState {
    pub fn to_context(&self) -> AppContext {
        AppContext {
            config: self.config.clone(),
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

    pub fn language_loader(&self, langs: &[String]) -> FluentLanguageLoader {
        let langs = langs
            .iter()
            .map(|lang| lang.parse().unwrap())
            .collect::<Vec<LanguageIdentifier>>();

        LANGUAGE_LOADER.select_languages(&langs)
    }

    pub fn lang(&self, loader: &FluentLanguageLoader) -> String {
        loader
            .current_languages()
            .iter()
            .find_map(|language| {
                println!("{}", LANGUAGES.contains(&language));
                if LANGUAGES.contains(&language) {
                    Some(language.to_string())
                } else {
                    None
                }
            })
            .unwrap_or(loader.fallback_language().to_string())
    }
}

#[derive(Deserialize)]
pub(crate) struct JwtClaims {
    pub sub: String,
}
