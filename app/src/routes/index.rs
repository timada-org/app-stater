use evento_query::QueryResult;
use i18n_embed_fl::fl;
use leptos::*;
use pikav_client::timada::SimpleEvent;
use starter_feed::{FeedMetadata, TagCount, UserFeed};

use crate::{
    components::*,
    state::{use_app, WebContext},
    subscriber::SubscribeEvent,
};

#[component]
pub fn IndexPage(
    tag: Option<String>,
    feeds: QueryResult<UserFeed>,
    popular_tags: Vec<TagCount>,
) -> impl IntoView {
    let app = use_app();

    view! {
        <Page head=|| {
            view! {
                <script
                    src="https://unpkg.com/htmx.org/dist/ext/sse.js"
                    crossorigin="anonymous"
                ></script>
            }
        }>
            {fl!(app.fl_loader, "index_hello-world")}
            <form
                hx-post=app.create_url("/_create-feed")
                hx-swap="innerHTML"
                hx-target="#form-response"
            >
                <input id="form-title" name="title" required/>
            </form> <div id="form-response"></div>
            <div hx-boost="true">
                {popular_tags
                    .iter()
                    .map(|tag| {
                        view! {
                            <a href=app.create_url(format!("?tag={}", & tag.tag))>{&tag.tag}</a>
                        }
                    })
                    .collect_view()}
            </div> <div hx-boost="true">
                <a href=app.create_url("")>Global feed</a>
                {tag.as_ref().map(|tag| view! { <span>"#" {tag}</span> })}
            </div> <div hx-ext="sse" sse-connect=app.create_sse_url("/index")>
                <div sse-swap="created" hx-target="#list-feeds" hx-swap="afterbegin"></div>
            </div> <div id="list-feeds">
                <Feeds tag query=feeds/>
            </div>
        </Page>
    }
}

#[component]
pub fn Feeds(tag: Option<String>, query: QueryResult<UserFeed>) -> impl IntoView {
    view! {
        <>
            {query
                .edges
                .into_iter()
                .map(|feed| {
                    let cursor = query
                        .page_info
                        .end_cursor
                        .to_owned()
                        .and_then(|cursor| {
                            if cursor == feed.cursor && query.page_info.has_next_page {
                                Some(cursor)
                            } else {
                                None
                            }
                        });
                    view! { <Feed tag=tag.to_owned() feed=feed.node cursor=cursor/> }
                })
                .collect_view()}
        </>
    }
}

#[component]
pub fn Feed(
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] hs: Option<&'static str>,
    feed: UserFeed,
    cursor: Option<String>,
    tag: Option<String>,
) -> impl IntoView {
    let app = use_app();

    let (hx_get, hx_trigger, hx_swap) = if let Some(cursor) = cursor {
        let tag = tag
            .map(|tag| format!("&tag={tag}"))
            .unwrap_or("".to_owned());
        (
            Some(app.create_url(format!("/_load-more?first=20&after={cursor}{tag}"))),
            Some("revealed"),
            Some("afterend"),
        )
    } else {
        (None, None, None)
    };

    view! {
        <div
            // {..attrs}
            _=hs
            id=format!("feed-{}", feed.id)
            hx-get=hx_get
            hx-trigger=hx_trigger
            hx-swap=hx_swap
        >
            <div>
                <div>
                    {feed.author} - {app.format_localized(&feed.created_at, "%A %e %B %Y, %T")}
                </div>
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

pub async fn subscribe(
    ctx: WebContext,
    pikav: pikav_client::Client,
    event: SubscribeEvent<UserFeed, FeedMetadata>,
) -> anyhow::Result<()> {
    if let SubscribeEvent::Created(feed, metadata) = event {
        let html = ctx.html(move || {
            view! {
                <Feed
                    feed
                    tag=None
                    cursor=None
                    hs="init remove @disabled from #form-title then call #form-title.focus()"
                />
            }
        });

        pikav.publish(vec![SimpleEvent {
            user_id: metadata.req_user.to_string(),
            topic: "index".into(),
            event: "created".into(),
            data: html,
        }])
    };

    Ok(())
}
