mod feed;
mod index;

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};

use evento_query::QueryArgs;
use index::{Feeds, IndexPage};
use leptos::*;
use serde::Deserialize;
use starter_feed::{CreateFeedInput, ListFeedsInput};
use validator::Validate;

pub use index::subscribe;

use crate::state::AppContext;

pub fn create_router() -> Router<()> {
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

async fn index(ctx: AppContext, Query(query): Query<IndexQuery>) -> impl IntoResponse {
    let input = ListFeedsInput {
        args: QueryArgs::forward::<String>(20, None),
        tag: query.tag.to_owned(),
    };

    let (feeds, popular_tags) = match tokio::try_join!(
        ctx.feed_query.list_feeds(input),
        ctx.feed_query.list_popular_tags()
    ) {
        Ok(res) => res,
        Err(e) => return ctx.internal_server_error(e).into_response(),
    };

    ctx.html(move || {
        view! {
            <IndexPage feeds popular_tags tag=query.tag prev_tag=query.prev_tag />
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
    Query(args): Query<QueryArgs>,
    Query(query): Query<LoadMoreQuery>,
) -> impl IntoResponse {
    let input = ListFeedsInput {
        args,
        tag: query.tag.to_owned(),
    };

    let feeds = match ctx.feed_query.list_feeds(input).await {
        Ok(res) => res,
        Err(e) => return ctx.internal_server_error(e).into_response(),
    };

    ctx.html(move || {
        view! { <Feeds tag=query.tag query=feeds/> }
    })
    .into_response()
}

async fn create_feed(ctx: AppContext, Form(input): Form<CreateFeedInput>) -> impl IntoResponse {
    if let Err(errors) = input.validate() {
        return ctx.unprocessable_entity(errors).into_response();
    }

    if let Err(e) = ctx.feed_cmd.create(&input).await {
        return ctx.internal_server_error(e).into_response();
    }

    ctx.html(move || ()).into_response()
}
