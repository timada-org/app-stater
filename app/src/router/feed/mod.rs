use crate::{component::Page, AppState};
use axum::{body::Body, extract::State, response::IntoResponse, routing::get, Router};
use i18n_embed_fl::fl;
use leptos::*;

pub async fn root(State(app): State<AppState>) -> impl IntoResponse {
    let language_loader = app.language_loader(&["fr".to_owned()]);

    app.render_to_string(|| {
        view! { <Page title="Feed">{fl!(language_loader, "hello-world-feed")}</Page> }
    })
}

pub fn create_router() -> Router<AppState, Body> {
    Router::new().route("/", get(root))
}
