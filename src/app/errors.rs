//! Generic error constructs to use for API error reponses

use anyhow::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::app::api_response;

/// A generic app error sent by all handlers on failure
///
/// Wraps an `anyhow::Error` to be converted into an API response
pub struct AppError(Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        log::error!("{}", self.0.to_string());
        api_response::error(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
