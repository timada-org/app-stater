mod index;

use axum::{routing::get, Router};
use index::*;

pub fn create_router() -> Router {
    Router::new().route("/", get(index))
}
