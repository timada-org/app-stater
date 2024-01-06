mod error;
mod index;

use axum::{Router, routing::get};

pub use error::*;

pub fn create_router() -> Router {
    Router::new().route("/", get(index::index))
}
