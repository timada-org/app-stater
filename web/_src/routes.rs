mod feed;
mod index;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};

// pub use index::subscribe;
use index::{Feeds, IndexPage};
use leptos::*;
use serde::Deserialize;
use starter_feed::{CreateFeedInput, ListFeedsInput, ListPopularTagsInput};

use crate::state::AppContext;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/_load-more", get(load_more))
        .route("/_create-feed", post(create_feed))
        .nest("/feed/:id", feed::create_router())
}
