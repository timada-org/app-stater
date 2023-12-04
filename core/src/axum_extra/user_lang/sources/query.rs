use crate::axum_extra::user_lang::UserLanguageSource;
use axum::{async_trait, extract::Query, http::request::Parts};
use std::collections::HashMap;

/// TBD
#[derive(Debug, Clone)]
pub struct QuerySource {
    /// TBD
    name: String,
}

impl QuerySource {
    /// TBD
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[async_trait]
impl UserLanguageSource for QuerySource {
    async fn languages_from_parts(&self, parts: &mut Parts) -> Vec<String> {
        let Some(query) = parts.extensions.get::<Query<HashMap<String, String>>>() else {
            return vec![];
        };

        let Some(lang) = query.get(self.name.as_str()) else {
            return vec![];
        };

        vec![lang.to_string()]
    }
}
