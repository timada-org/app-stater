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
    let input = ListFeedsInput {
        args: QueryArgs::forward::<String>(20, None),
        tag: tag_query.tag.to_owned(),
    };

    let (feeds, popular_tags) = match tokio::try_join!(
        ctx.feed_query.list_feeds(input),
        ctx.feed_query.list_popular_tags()
    ) {
        Ok(res) => res,
        Err(e) => return ctx.internal_server_error(e).into_response(),
    };

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
                    {tag_query.tag.as_ref().map(|tag| view! {<span>"#"{tag}</span>})}
                </div>
                <div id="list-feeds">
                    <Feeds tag=tag_query.tag query=feeds />
                </div>
            </Page>
        }
    })
    .into_response()
}
