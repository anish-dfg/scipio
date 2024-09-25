//! Cycles API.

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::context::Context;

mod controllers;
mod responses;

/// Documents the API for managing cycles
#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::fetch_cycles,
        controllers::delete_cycle,
    ),
    security(("http" = ["JWT"]))
)]
pub struct CyclesApi;

/// Builds the cycles API.
///
/// * `ctx`: The application context
pub async fn build(ctx: Arc<Context>) -> Router<()> {
    let guard1 = make_rbac(vec!["read:cycles".to_owned()]).await;
    let guard2 = make_rbac(vec!["delete:cycles".to_owned()]).await;

    let fetch_cycles = routing::get(controllers::fetch_cycles);
    let delete_cycle = routing::delete(controllers::delete_cycle);

    Router::new()
        .route("/", fetch_cycles)
        .route_layer(from_fn_with_state(ctx.clone(), guard1))
        .route("/:id", delete_cycle)
        .route_layer(from_fn_with_state(ctx.clone(), guard2))
        .with_state(ctx.clone())
}
