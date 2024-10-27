//! Data Exports API.

mod controllers;
mod requests;
mod responses;
mod workspace;

use std::sync::Arc;

use axum::extract::FromRef;
use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

struct ExportServices {
    pub storage_layer: Arc<dyn crate::services::storage::StorageService>,
    pub workspace: Arc<dyn crate::services::workspace::WorkspaceService>,
    pub mail: Arc<dyn crate::services::mail::MailService>,
}

impl FromRef<Arc<Services>> for ExportServices {
    fn from_ref(ctx: &Arc<Services>) -> Self {
        Self {
            storage_layer: ctx.storage_layer.clone(),
            workspace: ctx.workspace.clone(),
            mail: ctx.mail.clone(),
        }
    }
}

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
pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let export_workspace_guard = make_rbac(vec!["export:volunteers-workspace".to_owned()]).await;

    let export_users_to_workspace = routing::post(controllers::export_users_to_workspace);

    Router::new()
        .route("/:project_cycle_id/workspace", export_users_to_workspace)
        .route_layer(from_fn_with_state(ctx.clone(), export_workspace_guard))
        .with_state(ctx.clone())
}
