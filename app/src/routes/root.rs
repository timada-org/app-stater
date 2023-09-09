use axum::{extract::State, response::IntoResponse};
use i18n_embed_fl::fl;
use leptos::*;
use starter_core::axum_extra::UserLanguage;

use crate::components::*;

use super::AppState;

pub(super) async fn root(State(app): State<AppState>, lang: UserLanguage) -> impl IntoResponse {
    let lang_loader = app.language_loader(lang.preferred_languages());

    app.render_to_string(|| {
        view! { <Page>{fl!(lang_loader, "root_hello-world")}</Page> }
    })
}
