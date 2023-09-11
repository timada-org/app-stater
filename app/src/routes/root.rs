use axum::{extract::State, response::IntoResponse};
use i18n_embed_fl::fl;
use leptos::*;
use starter_core::axum_extra::UserLanguage;
use twa_jwks::axum::JwtPayload;

use crate::{components::*, state::JwtClaims};

use super::AppState;

pub(super) async fn root(
    State(app): State<AppState>,
    user_lang: UserLanguage,
    JwtPayload(_jwt): JwtPayload<JwtClaims>,
) -> impl IntoResponse {
    let fl_loader = app.language_loader(user_lang.preferred_languages());
    let lang = app.lang(&fl_loader);

    app.render_to_string(move || {
        view! { <Page lang=lang>{fl!(fl_loader, "root_hello-world")}</Page> }
    })
}
