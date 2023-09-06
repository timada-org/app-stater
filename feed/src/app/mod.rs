use axum::{body::Body, extract::State, response::IntoResponse, routing::get, Router};
use leptos::*;
use starter_components::*;
use starter_core::AppState;

pub async fn root(State(app): State<AppState>) -> impl IntoResponse {
    app.render_to_string(|| {
        view! { <Page title="Feed">"Feed hello world"</Page> }
    })
}

pub fn create_router() -> Router<AppState, Body> {
    Router::new().route("/", get(root))
}
