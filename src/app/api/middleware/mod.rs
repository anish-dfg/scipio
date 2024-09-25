//! Defines middleware for use in the application
//!
//! This should be split into multiple modules as the application grows.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::app::api_response;
use crate::app::context::Context;
use crate::app::errors::AppError;
use crate::services::auth::AuthData;

/// Middleware for role-based access control (RBAC).
///
/// * `header`: The authorization header containing the bearer token
/// * `request`: The request object from axum
/// * `next`: The next middleware in the chain
/// * `permissions`: The permissions required to access the route (such as `["read:volunteers"]`)
pub async fn rbac(
    State(ctx): State<Arc<Context>>,
    header: TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
    permissions: Vec<String>,
) -> Result<Response, AppError> {
    let authenticator = &ctx.authenticator;
    let token = header.0.token();

    let data = authenticator.authenticate(token).await?;

    match data {
        AuthData::Auth0(ref rbac_data) => {
            if permissions.iter().all(|p| rbac_data.permissions.contains(p)) {
                request.extensions_mut().insert(data);
                Ok(next.run(request).await)
            } else {
                Ok(api_response::error(StatusCode::FORBIDDEN, "unauthorized"))
            }
        }
        AuthData::Noop => Ok(api_response::success(StatusCode::OK, "noop")?),
    }
}

/// Higher order function to create a partially applied RBAC middleware with the given permissions.
///
/// * `permissions`: A vector of permissions required to access the route
pub async fn make_rbac(
    permissions: Vec<String>,
) -> impl Fn(
    State<Arc<Context>>,
    TypedHeader<Authorization<Bearer>>,
    Request,
    Next,
) -> Pin<Box<dyn Future<Output = Result<Response, AppError>> + Send + 'static>>
       + Clone {
    move |state, header, request, next| {
        Box::pin(rbac(state, header, request, next, permissions.clone()))
    }
}
