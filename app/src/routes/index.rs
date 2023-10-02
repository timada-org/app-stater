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
    prev_tag: Option<String>,
    feeds: QueryResult<UserFeed>,
    popular_tags: Vec<TagCount>,
) -> impl IntoView {
    let app = use_app();
    let index_css = app.create_css_url("index.css");
    let global_link_css = if tag.is_some() {
        "px-4 pb-2 relative bottom-[-1.3px]"
    } else {
        "text-info border-b-2 border-info relative bottom-[-1.3px] px-4 pb-2"
    };

    let global_link = tag
        .as_ref()
        .map(|tag| format!("?prev_tag={tag}"))
        .unwrap_or_default();

    view! {
        <Layout head=move || {
            view! {
                <>
                    <link rel="stylesheet" href=index_css crossorigin="anonymous" />

                    <HtmxSseScript/>
                <>
             }
        }>
            {fl!(app.fl_loader, "index_hello-world")}
            <form
                hx-post=app.create_url("/_create-feed")
                hx-swap="innerHTML"
                hx-target="#form-response"
            >
                <input id="form-title" name="title" required/>
            </form>
            <div id="form-response"></div>
            <div hx-ext="sse" sse-connect=app.create_sse_url("/index")>
                <div sse-swap="created" hx-target="#list-feeds" hx-swap="afterbegin"></div>
            </div>
            <div class="grid grid-cols-[auto_24rem] gap-4">
                <div>
                    <div class="border-b pb-2 mb-4" hx-boost="true">
                        <a class=global_link_css href=app.create_url(global_link)>Global feed</a>
                        {prev_tag.as_ref().map(|tag| view! { <a href=app.create_url(format!("?tag={}", &tag)) class="lowercase px-4 pb-2 relative bottom-[-1.3px]">"#" {tag}</a> })}
                        {tag.as_ref().map(|tag| view! { <span class="text-info border-b-2 border-info relative bottom-[-1.3px] lowercase px-4 pb-2">"#" {tag}</span> })}
                    </div>
                    <div id="list-feeds">
                        <Feeds backward=true tag query=feeds/>
                    </div>
                </div>
                <div hx-boost="true">
                    {popular_tags
                        .iter()
                        .map(|tag| {
                            view! {
                                <div class="badge badge-outline mr-2 mt-2 lowercase">
                                    <a href=app.create_url(format!("?tag={}", & tag.tag))>{&tag.tag}</a>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </Layout>
    }
}

#[component]
pub fn Feeds(
    tag: Option<String>,
    query: QueryResult<UserFeed>,
    #[prop(optional)] backward: bool,
) -> impl IntoView {
    view! {
        <>
            {query
                .edges
                .into_iter()
                .map(|feed| {
                    let start_cursor = query
                        .page_info
                        .end_cursor
                        .to_owned()
                        .and_then(|cursor| {
                            if backward && cursor == feed.cursor && query.page_info.has_next_page {
                                Some(cursor)
                            } else {
                                None
                            }
                        });
                    let end_cursor = query
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
                    view! { <Feed tag=tag.to_owned() feed=feed.node start_cursor end_cursor/> }
                })
                .collect_view()}
        </>
    }
}

#[component]
pub fn Feed(
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(optional)] hs: Option<&'static str>,
    feed: UserFeed,
    start_cursor: Option<String>,
    end_cursor: Option<String>,
    tag: Option<String>,
) -> impl IntoView {
    let app = use_app();

    let (hx_get, hx_trigger, hx_swap) = if let Some(end_cursor) = end_cursor {
        let tag = tag
            .map(|tag| format!("&tag={tag}"))
            .unwrap_or("".to_owned());
        (
            Some(app.create_url(format!("/_load-more?first=20&after={end_cursor}{tag}"))),
            Some("revealed"),
            Some("afterend"),
        )
    } else {
        (None, None, None)
    };
    
    // afterbegin

    view! {
        <div
            {..attrs}
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
                        "Read more..."
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
                    start_cursor=None
                    end_cursor=None
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
