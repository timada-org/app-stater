use axum::{extract::State, response::IntoResponse};
use i18n_embed_fl::fl;
use leptos::*;
use starter_core::axum_extra::UserLanguage;
use twa_jwks::axum::JwtPayload;

use crate::{components::*, state::JwtClaims};

use super::AppState;

pub(super) async fn root(State(app): State<AppState>, lang: UserLanguage, JwtPayload(_jwt): JwtPayload<JwtClaims>) -> impl IntoResponse {
    let fl_loader = app.language_loader(lang.preferred_languages());

    app.render_to_string(|| {
        view! { <Page>{fl!(fl_loader, "root_hello-world")}</Page> }
    })
}
