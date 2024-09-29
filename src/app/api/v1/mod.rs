//! Defines and builds the API for version 1 of the Pantheon API.

mod authz;
mod cycles;
mod data_exports;
mod data_imports;
mod jobs;
mod stats;
mod volunteers;

use std::sync::Arc;

use authz::AuthzApi;
use axum::Router;
use cycles::CyclesApi;
use data_exports::DataExportsApi;
use data_imports::DataImportsApi;
use jobs::JobsApi;
use stats::StatsApi;
use utoipa::OpenApi;
use volunteers::VolunteersApi;

use crate::app::state::Services;

/// API Documentation for version 1 of the Pantheon API.
#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/data-imports", api = DataImportsApi),
        (path = "/data-exports", api = DataExportsApi),
        (path = "/authz", api = AuthzApi),
        (path = "/cycles", api = CyclesApi),
        (path = "/jobs", api = JobsApi),
        (path = "/volunteers", api = VolunteersApi),
        (path = "/stats", api = StatsApi),
    ),
)]
pub struct V1Api;

/// Builds the API for version 1 of the Pantheon API.
///
/// * `ctx`: The application context
pub async fn build(services: Arc<Services>) -> Router<()> {
    let data_import_routes = data_imports::build(services.clone()).await;
    let data_export_routes = data_exports::build(services.clone()).await;
    let authz_routes = authz::build(services.clone()).await;
    let cycles_routes = cycles::build(services.clone()).await;
    let jobs_routes = jobs::build(services.clone()).await;
    let volunteers_routes = volunteers::build(services.clone()).await;
    let stats_routes = stats::build(services.clone()).await;

    Router::new()
        .nest("/data-imports", data_import_routes)
        .nest("/data-exports", data_export_routes)
        .nest("/authz", authz_routes)
        .nest("/cycles", cycles_routes)
        .nest("/jobs", jobs_routes)
        .nest("/volunteers", volunteers_routes)
        .nest("/stats", stats_routes)
}
