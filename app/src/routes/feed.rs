mod index;

use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use index::IndexPage;
use leptos::*;

use crate::state::AppContext;

pub fn create_router() -> Router<()> {
    Router::new().route("/", get(index))
}

async fn index(ctx: AppContext, Path((id,)): Path<(String,)>) -> impl IntoResponse {
    let feed = match ctx.feed_query.get_feed(id).await {
        Ok(feed) => feed,
        Err(e) => return ctx.internal_server_error_page(e).into_response(),
    };

    let Some(feed) = feed else {
        return ctx.not_found_page().into_response();
    };

    ctx.html(move || {
        view! { <IndexPage feed/> }
    })
    .into_response()
}
