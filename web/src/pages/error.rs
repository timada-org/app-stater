use askama::Template;
use askama_axum::{Response, IntoResponse};
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundPage;

#[derive(Template)]
#[template(path = "500.html")]
pub struct InternalServerErrorPage;

#[derive(Debug)]
pub enum Error {
    Command(evento::CommandError),
    Query(evento::QueryError),
}

impl From<evento::CommandError> for Error {
    fn from(value: evento::CommandError) -> Self {
        Error::Command(value)
    }
}

impl From<evento::QueryError> for Error {
    fn from(value: evento::QueryError) -> Self {
        Error::Query(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Command(err) => match err {
                evento::CommandError::Server(_) => (
                    StatusCode::NOT_FOUND,
                    [("X-Up-Target", ".errors")],
                    InternalServerErrorPage,
                )
                    .into_response(),
                evento::CommandError::Validation(errors) => (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    [("X-Up-Target", ".errors")],
                    format!("{errors:?}"),
                )
                    .into_response(),
                evento::CommandError::NotFound(_) => (
                    StatusCode::NOT_FOUND,
                    [("X-Up-Target", ".errors")],
                    NotFoundPage,
                )
                    .into_response(),
            },
            Error::Query(err) => match err {
                evento::QueryError::Server(_) => (
                    StatusCode::NOT_FOUND,
                    [("X-Up-Target", ".errors")],
                    InternalServerErrorPage,
                )
                    .into_response(),
                evento::QueryError::NotFound(_) => (
                    StatusCode::NOT_FOUND,
                    [("X-Up-Target", ".errors")],
                    NotFoundPage,
                )
                    .into_response(),
            },
        }
    }
}
