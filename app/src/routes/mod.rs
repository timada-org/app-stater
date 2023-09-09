mod feed;
mod root;

use axum::{body::Body, routing::get, Router};

use crate::state::AppState;

pub fn create_router() -> Router<AppState, Body> {
    Router::new()
        .route("/", get(root::root))
        .nest("/feed", feed::create_router())
}
