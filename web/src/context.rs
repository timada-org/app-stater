use std::sync::Arc;

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::Html,
    Extension, RequestPartsExt,
};
use evento::{Command, Query};
use evento_axum::UserLanguage;
use i18n_embed::{fluent::FluentLanguageLoader, LanguageLoader};
use serde::Deserialize;
use twa_jwks::axum::JwtPayloadOption;
use unic_langid::LanguageIdentifier;

use crate::{
    config::Config,
    i18n::{LANGUAGES, LANGUAGE_LOADER},
};

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub command: Command,
    pub query: Query,
    pub user_language: Option<String>,
    pub fl_loader: Option<Arc<FluentLanguageLoader>>,
    pub user_id: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Html<&'static str>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Ok(JwtPayloadOption(jwt_claims)) =
            JwtPayloadOption::<JwtClaims>::from_request_parts(parts, state).await
        else {
            return Err((StatusCode::BAD_REQUEST, Html("Bad Request")));
        };

        let Ok(user_language) = UserLanguage::from_request_parts(parts, state).await else {
            return Err((StatusCode::BAD_REQUEST, Html("Bad Request")));
        };

        let langs = user_language
            .preferred_languages()
            .iter()
            .map(|lang| lang.parse().unwrap_or_default())
            .collect::<Vec<LanguageIdentifier>>();

        let fl_loader = LANGUAGE_LOADER.select_languages(&langs);

        let user_language = fl_loader
            .current_languages()
            .iter()
            .find_map(|language| {
                if LANGUAGES.contains(language) {
                    Some(language.to_string())
                } else {
                    None
                }
            })
            .unwrap_or(fl_loader.fallback_language().to_string());

        let Extension(mut ctx) = parts
            .extract::<Extension<Context>>()
            .await
            .expect("Context not configured correctly");

        ctx.user_language = Some(user_language);
        ctx.fl_loader = Some(Arc::new(fl_loader));
        ctx.user_id = jwt_claims.map(|claims| claims.sub);

        Ok(ctx)
    }
}

#[derive(Clone)]
pub struct UserContext {
    pub config: Config,
    pub command: Command,
    pub query: Query,
    pub user_language: Option<String>,
    pub fl_loader: Option<Arc<FluentLanguageLoader>>,
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Html<&'static str>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = Context::from_request_parts(parts, state).await?;

        let Some(user_id) = ctx.user_id else {
            return Err((StatusCode::UNAUTHORIZED, Html("Unauthorized")));
        };

        Ok(UserContext {
            command: ctx.command,
            query: ctx.query,
            user_language: ctx.user_language,
            user_id,
            config: ctx.config,
            fl_loader: ctx.fl_loader,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JwtClaims {
    pub sub: String,
}
