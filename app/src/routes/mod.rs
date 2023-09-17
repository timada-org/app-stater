mod api;
mod component;
mod feed;
mod page;

use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router<(), Body> {
    Router::new()
        .route("/", get(page::root))
        .route("/_load-more", get(api::load_more))
        .route("/_create-feed", post(api::create_feed))
        .nest("/feed/:id", feed::create_router())
}
