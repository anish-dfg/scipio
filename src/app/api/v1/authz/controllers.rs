//! Controllers for the authorization API.

use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Extension;

use crate::app::api_response;
use crate::app::context::Context;
use crate::app::errors::AppError;
use crate::services::auth::AuthData;

/// Fetch user info from the token.
///
/// * `ctx`: The application context extracted as Axum state
/// * `auth`: The authentication data extracted as Axum extension
///
/// This information is not fetched every time a user makes a request because of rate limits.
#[utoipa::path(
    post,
    path = "/user",
    operation_id = "Get user",
    responses(
        (status = 200, description = "Succesfully fetch user info from IDP"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn user(
    State(ctx): State<Arc<Context>>,
    Extension(auth): Extension<AuthData>,
) -> Result<Response, AppError> {
    let authenticator = &ctx.authenticator;
    match auth {
        AuthData::Auth0(data) => {
            let token = &data.token;

            let user_info = authenticator.user_info(token).await?;
            Ok(api_response::success(StatusCode::OK, user_info)?)
        }
        AuthData::Noop => Ok(api_response::success(StatusCode::OK, "noop")?),
    }
}

/// Fetch user permissions from the token.
///
/// * `auth`: The authentication data extracted as Axum extension
/// * `ctx`: The application context extracted as Axum state
#[utoipa::path(
    post,
    path = "/permissions",
    operation_id = "Get permissions",
    responses(
        (status = 200, description = "Get user permissions"),
        (status = 401, description = "Unauthorized")
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn permissions(Extension(auth): Extension<AuthData>) -> Result<Response, AppError> {
    match auth {
        AuthData::Auth0(data) => Ok(api_response::success(StatusCode::OK, data.permissions)?),
        AuthData::Noop => Ok(api_response::success(StatusCode::OK, "noop")?),
    }
}
