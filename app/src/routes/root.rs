use axum::{response::IntoResponse, Form};
use i18n_embed_fl::fl;
use leptos::*;
use timada_starter_feed::CreateFeedInput;
use validator::Validate;
use tracing::error;

use crate::{
    components::*,
    state::{use_app, AppContext},
};

pub(super) async fn root(ctx: AppContext) -> impl IntoResponse {
    ctx.html(move || {
        let app = use_app();

        view! {
            <Page>
                {fl!(app.fl_loader, "root_hello-world")}
                <div id="form-errors"></div>
                <form
                    hx-post=app.create_url("/_create-feed")
                    hx-ext="response-targets"
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
    ctx: AppContext,
    Form(input): Form<CreateFeedInput>,
) -> impl IntoResponse {
    if let Err(e) = input.validate() {
        return ctx
            .bad_request(move || {
                view! { <div _="init wait 3s remove me">{e.to_string()}</div> }
            })
            .into_response();
    }

    let id = match ctx.feed.create(&input).await {
        Ok(events) => events[0].aggregate_id.to_owned(),
        Err(e) => {
            error!("{e}");

            return ctx
                .internal_server_error( move || {
                    let app = use_app();
                    view! { <div _="init wait 3s remove me">{fl!(app.fl_loader, "http-errors_500")}</div> }
                }).into_response();
        }
    };

    ctx.html(move || {
        view! { <li id=id>{input.title}</li> }
    })
    .into_response()
}
