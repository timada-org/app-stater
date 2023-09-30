use evento_query::QueryResult;
use leptos::*;
use starter_feed::UserFeed;

use crate::state::use_app;

#[component]
pub(super) fn Feeds(tag: Option<String>, query: QueryResult<UserFeed>) -> impl IntoView {
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
