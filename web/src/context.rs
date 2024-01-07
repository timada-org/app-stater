use askama_axum::{IntoResponse, Response};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::Html,
    Extension, RequestPartsExt,
};
use chrono::{DateTime, Locale, TimeZone};
use evento::{Command, CommandHandler, Query, QueryHandler};
use evento_axum::UserLanguage;
use i18n_embed::{fluent::FluentLanguageLoader, LanguageLoader};
use serde::Deserialize;
use std::{collections::HashMap, fmt, sync::Arc};
use tracing::{error, warn};
use twa_jwks::axum::JwtPayloadOption;
use unic_langid::LanguageIdentifier;
use validator::Validate;

use crate::{
    config::Config,
    i18n::{LANGUAGES, LANGUAGE_LOADER},
    pages::{InternalServerErrorPage, NotFoundPage},
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

impl Context {
    pub fn user_language(&self) -> String {
        self.user_language
            .to_owned()
            .expect("user_language not configured correctly")
    }

    pub fn fl_loader(&self) -> Arc<FluentLanguageLoader> {
        self.fl_loader
            .clone()
            .expect("fl_loader not configured correctly")
    }

    pub async fn execute<I: Validate + CommandHandler>(
        &self,
        input: I,
    ) -> Result<Option<HashMap<String, Vec<String>>>, Response> {
        let Err(err) = self.command.execute(self.user_language(), &input).await else {
            return Ok(None);
        };

        match err {
            evento::CommandError::Server(err) => {
                error!("{err}");

                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    InternalServerErrorPage::new(self.clone()),
                )
                    .into_response())
            }
            evento::CommandError::Validation(errors) => Ok(Some(errors)),
            evento::CommandError::NotFound(_) => {
                Err((StatusCode::NOT_FOUND, NotFoundPage::new(self.clone())).into_response())
            }
        }
    }

    pub async fn query<I: QueryHandler>(&self, input: I) -> Result<I::Output, Response> {
        self.query.execute(&input).await.map_err(|e| match e {
            evento::QueryError::Server(err) => {
                error!("{err}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    InternalServerErrorPage::new(self.clone()),
                )
                    .into_response()
            }
            evento::QueryError::NotFound(_) => {
                (StatusCode::NOT_FOUND, NotFoundPage::new(self.clone())).into_response()
            }
        })
    }

    pub fn format_localized<'a, Tz: TimeZone>(&self, dt: &'a DateTime<Tz>, fmt: &'a str) -> String
    where
        Tz::Offset: fmt::Display,
    {
        let locale = match self.user_language().as_str() {
            "en" => Locale::en_US,
            "fr" => Locale::fr_FR,
            locale => {
                warn!("{locale} not handle in AppContext.format_localized");

                Locale::en_US
            }
        };

        dt.format_localized(fmt, locale).to_string()
    }

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

    pub fn create_sse_url(&self, uri: impl Into<String>) -> String {
        format!("/pikav/{}{}", self.config.pikav.namespace, uri.into())
    }

    #[cfg(debug_assertions)]
    pub fn hot_reload(&self) -> bool {
        true
    }

    #[cfg(not(debug_assertions))]
    pub fn hot_reload(&self) -> bool {
        false
    }
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
    inner: Context,
    pub user_id: String,
}

impl UserContext {
    pub fn user_language(&self) -> String {
        self.inner.user_language()
    }

    pub fn fl_loader(&self) -> Arc<FluentLanguageLoader> {
        self.inner.fl_loader()
    }

    pub async fn execute<I: Validate + CommandHandler>(
        &self,
        input: I,
    ) -> Result<Option<HashMap<String, Vec<String>>>, Response> {
        self.inner.execute(input).await
    }

    pub async fn query<I: QueryHandler>(&self, input: I) -> Result<I::Output, Response> {
        self.inner.query(input).await
    }

    pub fn format_localized<'a, Tz: TimeZone>(&self, dt: &'a DateTime<Tz>, fmt: &'a str) -> String
    where
        Tz::Offset: fmt::Display,
    {
        self.inner.format_localized(dt, fmt)
    }

    pub fn create_url(&self, uri: impl Into<String>) -> String {
        self.inner.create_url(uri)
    }

    pub fn create_static_url(&self, uri: impl Into<String>) -> String {
        self.inner.create_static_url(uri)
    }

    pub fn create_sse_url(&self, uri: impl Into<String>) -> String {
        self.inner.create_sse_url(uri)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Html<&'static str>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let inner = Context::from_request_parts(parts, state).await?;

        let Some(user_id) = inner.user_id.to_owned() else {
            return Err((StatusCode::UNAUTHORIZED, Html("Unauthorized")));
        };

        Ok(UserContext { inner, user_id })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JwtClaims {
    pub sub: String,
}
