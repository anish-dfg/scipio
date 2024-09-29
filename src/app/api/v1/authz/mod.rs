//! Authorization service API.

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

mod controllers;

/// Documents the API for the authorization service.
#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::user,
        controllers::permissions
    ),
    security(("http" = ["JWT"]))
)]
pub struct AuthzApi;

/// Builds the authorization service API.
///
/// * `ctx`: The application context
pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let basic_guard = make_rbac(vec![]).await;

    let user = routing::post(controllers::user);
    let permissions = routing::post(controllers::permissions);

    Router::new()
        .route("/user", user)
        .route("/permissions", permissions)
        .route_layer(from_fn_with_state(ctx.clone(), basic_guard))
        .with_state(ctx.clone())
}
