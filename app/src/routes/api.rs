use axum::{extract::Query, response::IntoResponse, Form};
use evento_query::QueryArgs;
use leptos::*;
use serde::Deserialize;
use starter_feed::{CreateFeedInput, ListFeedsInput};
use validator::Validate;

use crate::state::AppContext;

use super::component::Feeds;

#[derive(Deserialize)]
pub struct TagQuery {
    tag: Option<String>,
}

pub(super) async fn load_more(
    ctx: AppContext,
    Query(args): Query<QueryArgs>,
    Query(tag_query): Query<TagQuery>,
) -> impl IntoResponse {
    let feeds = ctx
        .feed_query
        .list_feeds(ListFeedsInput {
            args,
            tag: tag_query.tag.to_owned(),
        })
        .await
        .unwrap();

    ctx.html(move || {
        view! { <Feeds tag=tag_query.tag query=feeds/> }
    })
}

pub(super) async fn create_feed(
    ctx: AppContext,
    Form(input): Form<CreateFeedInput>,
) -> impl IntoResponse {
    if let Err(errors) = input.validate() {
        return ctx.unprocessable_entity(errors).into_response();
    }

    if let Err(e) = ctx.feed_cmd.create(&input).await {
        return ctx.internal_server_error(e).into_response();
    }

    ctx.html(move || {
        view! { <div _="init add @disabled to #form-title then remove me">""</div> }
    })
    .into_response()
}
