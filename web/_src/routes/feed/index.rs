use leptos::*;
use starter_feed::UserFeed;

use crate::{components::Layout, state::use_app};

#[component]
pub fn IndexPage(feed: UserFeed) -> impl IntoView {
    let app = use_app();
    let index_css = app.create_css_url("feed/index.css");

    view! {
        <Layout head=|| view! {
            <>
                <link rel="stylesheet" href=index_css crossorigin="anonymous" />
            </>
        } title=&feed.title>
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
        </Layout>
    }
}
