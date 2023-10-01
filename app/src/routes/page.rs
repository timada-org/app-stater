use axum::{extract::Query, response::IntoResponse};
use evento_query::QueryArgs;
use i18n_embed_fl::fl;
use leptos::*;
use serde::Deserialize;
use starter_feed::ListFeedsInput;

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
            <Page attr:hx-ext="response-targets" head=|| view! {
                <script
                    src="https://unpkg.com/htmx.org/dist/ext/sse.js"
                    crossorigin="anonymous"
                ></script>
                <script
                    src="https://unpkg.com/htmx.org/dist/ext/response-targets.js"
                    crossorigin="anonymous"
                ></script>
            }>
                {fl!(app.fl_loader, "root_hello-world")}
                <form
                    hx-post=app.create_url("/_create-feed")
                    hx-swap="innerHTML"
                    hx-target="#form-response"
                    hx-target-4xx="#form-response"
                    hx-target-5xx="#form-response"
                >
                    <input id="form-title" name="title" minlength="3" maxlength="100" required />
                </form>
                <div id="form-response"></div>
                <div hx-boost="true">
                    {popular_tags.iter().map(|tag| view! {
                        <a href=app.create_url(format!("?tag={}", &tag.tag))>{&tag.tag}</a>
                    }).collect_view()}
                </div>
                <div hx-boost="true">
                    <a href=app.create_url("")>Global feed</a>
                    {tag_query.tag.as_ref().map(|tag| view! {<span>"#"{tag}</span>})}
                </div>
                <div hx-ext="sse" sse-connect=app.create_sse_url("/root")>
                    <div sse-swap="created" hx-target="#list-feeds" hx-swap="afterbegin"></div>
                </div>
                <div id="list-feeds">
                    <Feeds tag=tag_query.tag query=feeds />
                </div>
            </Page>
        }
    })
    .into_response()
}
