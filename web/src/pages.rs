mod error;
mod feed;
mod index;

use axum::{
    routing::{get, post},
    Router,
};

pub use error::*;
use evento::Rule;
use starter_feed::FeedRule;

use self::index::*;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/_create-feed", post(create_feed))
        .route("/_load-more", get(load_more))
        .nest("/feed/:id", feed::create_router())
}

pub fn rules() -> Vec<Rule> {
    vec![Rule::new(FeedRule::FeedDetails).handler("feed/**", index::IndexFeedHandler)]
}
