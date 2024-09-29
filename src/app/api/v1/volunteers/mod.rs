use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

mod controllers;
mod responses;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::fetch_volunteers_by_cycle
    ),
    security(("http" = ["JWT"]))
)]
pub struct VolunteersApi;

pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let guard1 = make_rbac(vec!["read:volunteers".to_owned()]).await;

    let fetch_volunteers_by_cycle = routing::get(controllers::fetch_volunteers_by_cycle);

    Router::new()
        .route("/:project_cycle_id", fetch_volunteers_by_cycle)
        .route_layer(from_fn_with_state(ctx.clone(), guard1))
        .with_state(ctx.clone())
}
