//! Data Exports API.

mod controllers;
mod export_users_to_workspace;
mod requests;
mod responses;

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::context::Context;

/// Documents the API for data exports
#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::export_users_to_workspace,
    ),
    security(("http" = ["JWT"]))
)]
pub struct DataExportsApi;

/// Builds the data exports API.
///
/// * `ctx`: The application context
pub async fn build(ctx: Arc<Context>) -> Router<()> {
    let guard1 = make_rbac(vec![]).await;

    let export_users_to_workspace = routing::post(controllers::export_users_to_workspace);

    Router::new()
        .route("/:project_cycle_id/workspace", export_users_to_workspace)
        .route_layer(from_fn_with_state(ctx.clone(), guard1))
        .with_state(ctx.clone())
}
