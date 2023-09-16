use axum::{extract::Query, response::IntoResponse};
use evento_query::QueryArgs;
use i18n_embed_fl::fl;
use leptos::*;
use serde::Deserialize;
use timada_starter_feed::ListFeedsInput;

use crate::{
    components::*,
    state::{use_app, AppContext},
};

use super::component::Feeds;

#[derive(Deserialize)]
pub struct TagQuery {
    tag: Option<String>,
}

pub(super) async fn root(ctx: AppContext, Query(tag_query): Query<TagQuery>) -> impl IntoResponse {
    let feeds = ctx
        .feed_query
        .list_feeds(ListFeedsInput {
            args: QueryArgs::forward::<String>(20, None),
            tag: tag_query.tag.to_owned(),
        })
        .await
        .unwrap();

    let popular_tags = ctx.feed_query.list_popular_tags().await.unwrap();

    ctx.html(move || {
        let app = use_app();

        view! {
            <Page>
                {fl!(app.fl_loader, "root_hello-world")}
                <div id="form-errors"></div>
                <form
                    hx-post=app.create_url("/_create-feed")
                    hx-ext="response-targets"
                    hx-swap="beforeend"
                    hx-target="#list-feeds"
                    hx-target-4xx="#form-errors"
                    hx-target-5xx="#form-errors"
                >
                    <input name="title" minlength="3" maxlength="100" required />
                </form>
                <div hx-boost="true">
                    {popular_tags.iter().map(|tag| view! {
                        <a href=app.create_url(format!("?tag={}", &tag.tag))>{&tag.tag}</a>
                    }).collect_view()}
                </div>
                <div hx-boost="true">
                    <a href=app.create_url("")>Global feed</a>
                </div>
                <div id="list-feeds">
                    <Feeds tag=tag_query.tag query=feeds />
                </div>
            </Page>
        }
    })
    .into_response()
}
