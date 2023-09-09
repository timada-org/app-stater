use axum::{body::Body, extract::State, response::IntoResponse, routing::get, Router};
use i18n_embed_fl::fl;
use leptos::*;
use starter_core::axum_extra::UserLanguage;

use crate::{components::Page, AppState};

pub async fn root(State(app): State<AppState>, lang: UserLanguage) -> impl IntoResponse {
    let lang_loader = app.language_loader(lang.preferred_languages());

    app.render_to_string(|| {
        view! {
            <Page title="Feed">{fl!(lang_loader, "feed-root_hello-world")}</Page>
        }
    })
}

pub fn create_router() -> Router<AppState, Body> {
    Router::new().route("/", get(root))
}
