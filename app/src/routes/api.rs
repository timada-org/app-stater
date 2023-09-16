use axum::{extract::Query, response::IntoResponse, Form};
use evento_query::QueryArgs;
use i18n_embed_fl::fl;
use leptos::*;
use serde::Deserialize;
use timada_starter_feed::{CreateFeedInput, ListFeedsInput};
use tracing::error;
use validator::Validate;

use crate::state::{use_app, AppContext};

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
    if let Err(e) = input.validate() {
        return ctx
            .bad_request(move || {
                view! { <div _="init wait 3s remove me">{e.to_string()}</div> }
            })
            .into_response();
    }

    let id = match ctx.feed_cmd.create(&input).await {
        Ok(events) => events[0].aggregate_id.to_owned(),
        Err(e) => {
            error!("{e}");

            return ctx
                .internal_server_error( move || {
                    let app = use_app();
                    view! { <div _="init wait 3s remove me">{fl!(app.fl_loader, "http-errors_500")}</div> }
                }).into_response();
        }
    };

    ctx.html(move || {
        view! { <div id=id>"Creating '" {input.title} "' ..."</div> }
    })
    .into_response()
}
