use leptos::*;
use starter_feed::UserFeed;

use crate::components::Page;

#[component]
pub fn IndexPage(feed: UserFeed) -> impl IntoView {
    view! {
        <Page head=|| () title=&feed.title>
            <div>
                <div>{feed.author}</div>
                <div>{feed.total_likes}</div>
            </div>
            <article>
                <h2>{feed.title}</h2>
                <p>{feed.content}</p>
            </article>
            <div>
            {feed.tags.iter().map(|tag| view! {
                <span>{tag}</span>
            }).collect_view()}
            </div>
        </Page>
    }
}
