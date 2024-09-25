mod controllers;
mod middleware;
pub(in crate::app) mod v1;

use std::sync::Arc;

use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::context::Context;

#[derive(OpenApi)]
#[openapi(paths(controllers::health))]
pub struct TopLevelApi;

/// Builds the API router.
///
/// * `ctx`: The application context
pub async fn build(ctx: Arc<Context>) -> Router<()> {
    let v1_routes = v1::build(ctx.clone()).await;
    Router::new()
        .with_state(ctx.clone())
        .nest("/v1", v1_routes)
        .route("/health", routing::get(controllers::health))
}
