mod controllers;
mod middleware;
pub(in crate::app) mod v1;

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use middleware::make_rbac;
use utoipa::OpenApi;

use crate::app::api::v1::V1Api;
use crate::app::state::Services;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::health,
        controllers::services,
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
    let guard1 = make_rbac(vec![]).await;

    let v1_routes = v1::build(services.clone()).await;
    Router::new()
        .route("/services", routing::get(controllers::services))
        .route_layer(from_fn_with_state(services.clone(), guard1))
        .route("/health", routing::get(controllers::health))
        .with_state(services.clone())
        .nest("/v1", v1_routes)
}
