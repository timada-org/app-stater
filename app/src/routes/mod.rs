mod feed;
mod root;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<(), Body> {
    Router::new()
        .route("/", get(root::root))
        .route("/_create-feed", post(root::root_create_feed))
        .nest("/feed", feed::create_router())
}
