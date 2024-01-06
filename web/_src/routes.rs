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

#[derive(Deserialize)]
pub struct IndexQuery {
    tag: Option<String>,
    prev_tag: Option<String>,
}

async fn index(
    ctx: AppContext,
    axum::extract::Query(input): axum::extract::Query<IndexQuery>,
    axum::extract::Query(list_feeds_input): axum::extract::Query<ListFeedsInput>,
    Extension(query): Extension<evento::Query>,
) -> impl IntoResponse {
    let (feeds, popular_tags) = match tokio::try_join!(
        query.execute::<_, Render>(&list_feeds_input),
        query.execute::<_, Render>(&ListPopularTagsInput)
    ) {
        Ok(res) => res,
        Err(e) => {
            return match e {
                evento::QueryError::Server(e) | evento::QueryError::NotFound(e) => {
                    ctx.internal_server_error(e).into_response()
                }
            }
        }
    };
    ctx.html(move || {
        view! {
            <IndexPage feeds popular_tags tag=input.tag prev_tag=input.prev_tag />
        }
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct LoadMoreQuery {
    tag: Option<String>,
}

async fn load_more(
    ctx: AppContext,
    axum::extract::Query(query): axum::extract::Query<LoadMoreQuery>,
    feeds: Query<ListFeedsInput>,
) -> impl IntoResponse {
    ctx.html(move || {
        view! { <Feeds tag=query.tag query=feeds.output/> }
    })
}

async fn create_feed(ctx: AppContext, cmd: Command<CreateFeedInput>) -> impl IntoResponse {
    if let Err(_errors) = cmd.output {
        return (StatusCode::UNPROCESSABLE_ENTITY, "").into_response();
    }

    // if let Err(e) = ctx.feed_cmd.create(&input).await {
    //     return ctx.internal_server_error(e).into_response();
    // }

    ctx.html(move || ()).into_response()
}
