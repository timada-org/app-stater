use axum::{body::Body, extract::Path, response::IntoResponse, routing::get, Router};
use leptos::*;

use crate::{components::Page, state::AppContext};

pub async fn root(ctx: AppContext, Path((id,)): Path<(String,)>) -> impl IntoResponse {
    let Some(feed) = ctx.feed_query.get_feed(id).await.unwrap() else {
        return ctx.not_found(move || view! {"Not Found"}).into_response();
    };

    ctx.html(move || {
        view! {
            <Page title=&feed.title>
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
    })
    .into_response()
}

pub fn create_router() -> Router<(), Body> {
    Router::new().route("/", get(root))
}
