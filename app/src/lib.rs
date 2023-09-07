mod component;
mod config;
mod context;
mod router;

use anyhow::Result;
use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use component::Page;
use leptos::*;
use router::AppState;
use rust_embed::RustEmbed;
use tracing::info;

use crate::config::Config;

pub async fn serve() -> Result<()> {
    let config = Config::new()?;
    let app_state = AppState {
        config: config.app.clone(),
    };

    let router = router::create_router();

    let app = match config.app.base_url {
        Some(base_url) => Router::new().nest(&base_url, router),
        _ => router,
    }
    .fallback(get(static_handler))
    .with_state(app_state);

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
struct Asset;

async fn static_handler(uri: Uri, State(app): State<AppState>) -> impl IntoResponse {
    let uri = uri.to_string();
    let path = app
        .config
        .base_url
        .map(|base_url| {
            let mut uri = uri.to_owned();
            uri.replace_range(0..base_url.len(), "");

            uri
        })
        .unwrap_or(uri);

    match Asset::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
