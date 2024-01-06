mod index;

use anyhow::anyhow;
use axum::Extension;
use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use index::IndexPage;
use leptos::*;
use starter_feed::GetFeedInput;

use crate::state::AppContext;

pub fn create_router() -> Router<()> {
    Router::new().route("/", get(index))
}

async fn index(
    ctx: AppContext,
    Path((id,)): Path<(String,)>,
    Extension(query): Extension<evento::Query>,
) -> impl IntoResponse {
    let Ok(feed) = query
        .execute::<_, crate::Render>(&GetFeedInput { id })
        .await
    else {
        return ctx
            .internal_server_error_page(anyhow!("Something went wrong"))
            .into_response();
    };

    let Some(feed) = feed else {
        return ctx.not_found_page().into_response();
    };

    ctx.html(move || {
        view! { <IndexPage feed/> }
    })
    .into_response()
}
