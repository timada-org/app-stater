mod components;
mod config;
mod i18n;
mod routes;
mod state;

use anyhow::Result;
use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use evento::PgEngine;
use leptos::*;
use rust_embed::RustEmbed;
use sqlx::PgPool;
use starter_core::axum_extra::{AcceptLanguageSource, QuerySource, UserLanguage};
use tracing::info;
use twa_jwks::JwksClient;

use crate::{config::Config, state::AppState};

pub async fn serve() -> Result<()> {
    let config = Config::new()?;
    let state_config = config.app.clone();

    let jwks = JwksClient::build(config.app.jwks_url).await?;
    let db = PgPool::connect(&config.dsn).await?;
    let pikva_client = pikav_client::Client::new(pikav_client::ClientOptions {
        url: config.app.pikav.url.to_owned(),
        namespace: config.app.pikav.namespace.to_owned(),
    })?;

    sqlx::migrate!("../migrations")
        .set_locking(false)
        .run(&db)
        .await?;

    let projection = PgEngine::new_prefix(db.clone(), "projection")
        .name(format!("{}-projection", config.region))
        .data(pikva_client)
        .data(state_config.clone())
        .subscribe(routes::subscriber())
        .run(config.app.evento_delay.unwrap_or(30))
        .await?;

    let evento = PgEngine::new(db.clone())
        .name(&config.region)
        .data(projection)
        .subscribe(timada_starter_feed::feeds_subscriber())
        .subscribe(timada_starter_feed::tags_count_subscriber())
        .run(config.app.evento_delay.unwrap_or(30))
        .await?;

    let router = routes::create_router();

    let app = match config.app.base_url {
        Some(base_url) => Router::new().nest(&base_url, router),
        _ => router,
    }
    .fallback(get(static_handler))
    .layer(Extension(
        UserLanguage::config()
            .add_source(QuerySource::new("lang"))
            .add_source(AcceptLanguageSource)
            .build(),
    ))
    .layer(Extension(jwks))
    .layer(Extension(AppState {
        config: state_config,
        evento,
        db,
    }));

    let addr = config.app.addr.parse()?;

    info!("app listening on http://{}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "public/"]
#[prefix = "/static/"]
struct Assets;

async fn static_handler(uri: Uri, Extension(app): Extension<AppState>) -> impl IntoResponse {
    let uri = uri.to_string();
    let path = app
        .config
        .base_url
        .map(|base_url| {
            let mut uri = uri.to_owned();

            if uri.starts_with(&base_url) {
                uri.replace_range(0..base_url.len(), "");
            }

            uri
        })
        .unwrap_or(uri);

    match Assets::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
