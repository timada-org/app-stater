use axum::{extract::Query, response::IntoResponse, Form};
use evento_query::QueryArgs;
use i18n_embed_fl::fl;
use leptos::*;
use serde::Deserialize;
use timada_starter_feed::{CreateFeedInput, ListFeedsInput, UserFeed};
use tracing::error;
use validator::Validate;

use crate::{
    components::*,
    state::{use_app, AppContext},
};

#[derive(Deserialize)]
pub struct TagQuery {
    tag: Option<String>,
}

pub(super) async fn root(
    ctx: AppContext,
    Query(args): Query<QueryArgs>,
    Query(tag_arg): Query<TagQuery>,
) -> impl IntoResponse {
    let display_page = args.first.is_none()
        && args.after.is_none()
        && args.before.is_none()
        && args.last.is_none();

    let args = if display_page {
        QueryArgs::forward::<String>(20, None)
    } else {
        args
    };

    let feeds = ctx
        .feed_query
        .list_feeds(ListFeedsInput {
            args,
            tag: tag_arg.tag.to_owned(),
        })
        .await
        .unwrap();

    let list_feeds_view = move || {
        view! {
            <>
                {feeds
                    .edges
                    .into_iter()
                    .map(|feed| {
                        view! {
                            <Feed
                                tag=tag_arg.tag.to_owned()
                                feed=feed.node
                                cursor=feeds
                                    .page_info
                                    .end_cursor
                                    .to_owned()
                                    .and_then(|cursor| {
                                        if cursor == feed.cursor { Some(cursor) } else { None }
                                    })
                            />
                        }
                    })
                    .collect_view()}
            </>
        }
    };

    if !display_page {
        return ctx.html(move || list_feeds_view()).into_response();
    }

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
                    {list_feeds_view()}
                </div>
            </Page>
        }
    })
    .into_response()
}

pub(super) async fn root_create_feed(
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

#[component]
fn Feed(feed: UserFeed, cursor: Option<String>, tag: Option<String>) -> impl IntoView {
    let app = use_app();

    let (hx_get, hx_trigger, hx_swap) = if let Some(cursor) = cursor {
        let tag = tag
            .map(|tag| format!("&tag={tag}"))
            .unwrap_or("".to_owned());
        (
            Some(app.create_url(format!("?first=20&after={cursor}{tag}"))),
            Some("revealed"),
            Some("afterend"),
        )
    } else {
        (None, None, None)
    };

    view! {
        <div id=format!("feed-{}", feed.id) hx-get=hx_get hx-trigger=hx_trigger hx-swap=hx_swap>
            <div>
                <div>{feed.author} - {app.format_localized(&feed.created_at, "%A %e %B %Y, %T")}</div>
                <div>{feed.total_likes}</div>
            </div>
            <article>
                <h2>{feed.title}</h2>
                <p>
                    {feed.content_short} ...
                    <a hx-boost="true" href=app.create_url(format!("/feed/{}", & feed.id))>
                        "Read more"
                    </a>
                </p>
            </article>
            <div>{feed.tags.iter().map(|tag| view! { <span>{tag}</span> }).collect_view()}</div>
        </div>
    }
}
