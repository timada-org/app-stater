use axum::{extract::Path, response::IntoResponse};
use leptos::*;

use crate::{components::*, state::AppContext};

pub async fn root(ctx: AppContext, Path((id,)): Path<(String,)>) -> impl IntoResponse {
    let feed = match ctx.feed_query.get_feed(id).await {
        Ok(feed) => feed,
        Err(e) => return ctx.internal_server_error_page(e).into_response(),
    };

    let Some(feed) = feed else {
        return ctx.not_found_page().into_response();
    };

    ctx.html(move || {
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
    })
    .into_response()
}
