use crate::axum_extra::user_lang::UserLanguageSource;
use axum::{async_trait, extract::Path, http::request::Parts};
use std::collections::HashMap;

/// TBD
#[derive(Debug, Clone)]
pub struct PathSource {
    /// TBD
    name: String,
}

impl PathSource {
    /// TBD
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[async_trait]
impl UserLanguageSource for PathSource {
    async fn languages_from_parts(&self, parts: &mut Parts) -> Vec<String> {
        let Some(path) = parts.extensions.get::<Path<HashMap<String, String>>>() else {
            return vec![];
        };

        let Some(lang) = path.get(self.name.as_str()) else {
            return vec![];
        };

        vec![lang.to_string()]
    }
}
