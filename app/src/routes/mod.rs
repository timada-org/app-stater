mod feed;
mod root;

use axum::{body::Body, routing::{get, post}, Router};

use crate::state::AppState;

pub fn create_router() -> Router<AppState, Body> {
    Router::new()
        .route("/", get(root::root))
        .route("/_create-feed", post(root::root_create_feed))
        .nest("/feed", feed::create_router())
}
