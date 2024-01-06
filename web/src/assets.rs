use askama_axum::IntoResponse;
use axum::http::{header, StatusCode, Uri};
use rust_embed::RustEmbed;

use crate::{context::Context, pages::NotFoundPage};

#[derive(RustEmbed)]
#[folder = "public/"]
#[prefix = "/static/"]
struct Assets;

pub async fn static_handler(uri: Uri, ctx: Context) -> impl IntoResponse {
    let uri = uri.to_string();
    let path = ctx
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

    if !path.starts_with("/static/") {
        return (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html")],
            NotFoundPage,
        )
            .into_response();
    }

    match Assets::get(path.as_str()) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
    }
}
