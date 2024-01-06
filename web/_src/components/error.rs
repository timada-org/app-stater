use i18n_embed_fl::fl;
use leptos::*;
use std::fmt::Write;
use tracing::{error, warn};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::state::use_app;

#[component]
pub fn InternalServerErrorAlert() -> impl IntoView {
    let app = use_app();

    view! {
        <div>
            <h1>{fl!(app.fl_loader, "components_page_internal-server-error_title")}</h1>
            <p>{fl!(app.fl_loader, "components_page_internal-server-error_content")}</p>
            <a href=app
                .create_url(
                    "",
                )>{fl!(app.fl_loader, "components_page_internal-server-error_return_home")}</a>
        </div>
    }
}

#[component]
pub fn UnprocessableEntityAlert(errors: ValidationErrors) -> impl IntoView {
    view! {
        <ul>
            {errors.errors().iter().map(|(path, err)| display_errors(err, path)).collect_view()}
        </ul>
    }
}

fn display_error(err: &ValidationError) -> impl IntoView {
    let app = use_app();

    if let Some(msg) = err.message.as_ref() {
        return view! { <span>{msg.clone()}</span> };
    }

    let message = match err.code.to_string().as_str() {
        "email" => fl!(app.fl_loader, "validator_email"),
        "required" => fl!(app.fl_loader, "validator_required"),
        "phone" => fl!(app.fl_loader, "validator_phone"),
        "length" => {
            match (
                err.params.get("min").and_then(|v| v.as_i64()),
                err.params.get("max").and_then(|v| v.as_i64()),
                err.params.get("equal").and_then(|v| v.as_i64()),
            ) {
                (None, None, Some(equal)) => {
                    fl!(app.fl_loader, "validator_length_equal", equal = equal)
                }
                (None, Some(max), None) => fl!(app.fl_loader, "validator_length_max", max = max),
                (Some(min), None, None) => fl!(app.fl_loader, "validator_length_min", min = min),
                (Some(min), Some(max), None) => fl!(
                    app.fl_loader,
                    "validator_length_min-max",
                    min = min,
                    max = max
                ),
                _ => err.to_string(),
            }
        }
        _ => {
            warn!(
                "{} not translate in UnprocessableEntityAlert.display_error",
                err.code
            );

            err.to_string()
        }
    };

    view! { <span>{message}</span> }
}

fn display_struct(errs: &ValidationErrors, path: &str) -> impl IntoView {
    let mut full_path = String::new();

    if let Err(e) = write!(&mut full_path, "{}.", path) {
        error!("{e}");
    }

    let base_len = full_path.len();
    let mut views = vec![];

    for (path, err) in errs.errors() {
        if let Err(e) = write!(&mut full_path, "{}", path) {
            error!("{e}");
        }

        views.push(display_errors(err, &full_path));
        full_path.truncate(base_len);
    }

    views.collect_view()
}

fn display_errors(errs: &ValidationErrorsKind, path: &str) -> impl IntoView {
    match errs {
        ValidationErrorsKind::Field(errs) => view! {
            {errs
                .iter()
                .map(|err| {
                    view! {
                        <li>
                            <span>{path.to_owned()}</span>
                            :
                            {display_error(err)}
                        </li>
                    }
                })
                .collect_view()}
        }
        .into_view(),
        ValidationErrorsKind::Struct(errs) => display_struct(errs, path).into_view(),
        ValidationErrorsKind::List(errs) => {
            let mut full_path = String::new();

            if let Err(e) = write!(&mut full_path, "{}", path) {
                error!("{e}");
            }

            let base_len = full_path.len();
            let mut views = vec![];

            for (idx, err) in errs.iter() {
                if let Err(e) = write!(&mut full_path, "[{}]", idx) {
                    error!("{e}");
                }

                views.push(display_struct(err, &full_path));
                full_path.truncate(base_len);
            }

            views.collect_view()
        }
    }
}
