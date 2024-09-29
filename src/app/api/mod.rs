mod controllers;
mod middleware;
pub(in crate::app) mod v1;

use std::sync::Arc;

use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::v1::V1Api;
use crate::app::state::Services;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::health,
    ),
    nest(
        (path="/v1", api = V1Api)
    )
)]
pub struct Api;

/// Builds the API router.
///
/// * `services`: The application services
pub async fn build(services: Arc<Services>) -> Router<()> {
    let v1_routes = v1::build(services.clone()).await;
    Router::new()
        .with_state(services.clone())
        .nest("/v1", v1_routes)
        .route("/health", routing::get(controllers::health))
}
