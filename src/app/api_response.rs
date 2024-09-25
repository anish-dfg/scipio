//! Utilities to build responses to API requests

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::Value;

/// Wrap data in a JSON object if it is a string
///
/// * `data`: The data to (maybe) wrap
fn wrap_data<T: Serialize>(data: T) -> Result<Value> {
    let value = serde_json::to_value(data)?;
    match value {
        Value::String(_) => Ok(serde_json::json!({"msg": value})),
        _ => Ok(value),
    }
}

/// Build a response to a successful api request
///
/// * `code`: A HTTP 2xx status code
/// * `data`: Any data to send back to the client
pub fn success<T: Serialize>(code: StatusCode, data: T) -> Result<Response> {
    let data = wrap_data(data)?;
    Ok((code, Json(data)).into_response())
}

pub fn no_content() -> Response {
    (StatusCode::NO_CONTENT,).into_response()
}

/// Build a response to a failed api request
///
/// * `code`: A HTTP 4xx or 5xx status code
/// * `msg`: A failure message to send back to the client
pub fn error(code: StatusCode, msg: &str) -> Response {
    (code, Json(serde_json::json!({"error": msg}))).into_response()
}
