mod page;

use axum::{body::Body, routing::get, Router};

pub fn create_router() -> Router<(), Body> {
    Router::new().route("/", get(page::root))
}
