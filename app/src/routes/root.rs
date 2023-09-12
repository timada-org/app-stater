use axum::{extract::State, response::IntoResponse, Form};
use http::{header, HeaderValue, StatusCode};
use i18n_embed_fl::fl;
use leptos::*;
use starter_core::axum_extra::UserLanguage;
use timada_starter_feed::{create_feed, CreateFeedInput};
use twa_jwks::axum::JwtPayload;
use validator::Validate;

use crate::{components::*, context::use_app, state::JwtClaims};

use super::AppState;

pub(super) async fn root(
    State(app): State<AppState>,
    user_lang: UserLanguage,
    JwtPayload(_jwt): JwtPayload<JwtClaims>,
) -> impl IntoResponse {
    let fl_loader = app.language_loader(user_lang.preferred_languages());
    let lang = app.lang(&fl_loader);

    app.html(move || {
        let app = use_app();
        view! {
            <Page lang=lang>
                {fl!(fl_loader, "root_hello-world")}
                <div id="form-errors"></div>
                <form
                    hx-post=app.create_url("/_create-feed")
                    hx-swap="beforeend"
                    hx-target="#list-feeds"
                    hx-target-4xx="#form-errors"
                    hx-target-5xx="#form-errors"
                >
                    <input name="title" minlength="3" maxlength="100" required />
                </form>
                <ul id="list-feeds">

                </ul>
            </Page>
        }
    })
}

pub(super) async fn root_create_feed(
    State(app): State<AppState>,
    user_lang: UserLanguage,
    JwtPayload(_jwt): JwtPayload<JwtClaims>,
    Form(input): Form<CreateFeedInput>,
) -> impl IntoResponse {
    let fl_loader = app.language_loader(user_lang.preferred_languages());

    if let Err(e) = input.validate() {
        return app
            .bad_request(move || {
                view! { <div _="init wait 3s remove me">{e.to_string()}</div> }
            })
            .into_response();
    }

    let id = match create_feed(&app.evento, &input).await {
        Ok(events) => events[0].aggregate_id.to_owned(),
        Err(e) => {
            error!("{e}");

            return app
                .internal_server_error(move || {
                    view! { <div _="init wait 3s remove me">{fl!(fl_loader, "http-errors_500")}</div> }
                })
                .into_response();
        }
    };

    app.html(move || {
        view! { <li id=id>{input.title}</li> }
    })
    .into_response()
}
