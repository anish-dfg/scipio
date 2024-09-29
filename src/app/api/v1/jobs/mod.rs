mod controllers;
mod responses;

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::fetch_jobs,
        controllers::cancel_job,
    ),
    security(("http" = ["JWT"]))
)]
pub struct JobsApi;

pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let guard1 = make_rbac(vec!["read:jobs".to_owned()]).await;

    let fetch_jobs = routing::get(controllers::fetch_jobs);
    let cancel_job = routing::post(controllers::cancel_job);

    Router::new()
        .route("/", fetch_jobs)
        .route("/cancel/:job_id", cancel_job)
        .route_layer(from_fn_with_state(ctx.clone(), guard1))
        .with_state(ctx.clone())
}
