use axum::{body::Body, response::IntoResponse, routing::get, Router};
use i18n_embed_fl::fl;
use leptos::*;

use crate::{
    components::Page,
    state::{use_app, AppContext},
};

pub async fn root(ctx: AppContext) -> impl IntoResponse {
    ctx.html(|| {
        let app = use_app();

        view! {
            <Page title="Feed">{fl!(app.fl_loader, "feed-root_hello-world")}</Page>
        }
    })
}

pub fn create_router() -> Router<(), Body> {
    Router::new().route("/", get(root))
}
