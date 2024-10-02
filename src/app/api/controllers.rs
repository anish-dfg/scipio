use std::sync::Arc;

use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;

use crate::app::api_response;
use crate::app::errors::AppError;
use crate::app::state::Services;

/// The health check endpoint
///
/// path: `/api/health`
///
/// This endpoint should be exposed for Kubernetes liveness and readiness probes.
#[utoipa::path(
        get,
        path = "/health",
        operation_id = "Check health",
        responses(
            (status = 200, description = "Check API health")
        )
    )]
pub async fn health() -> Result<Response, AppError> {
    Ok(api_response::success(StatusCode::OK, "healthy")?)
}

#[utoipa::path(
        get,
        path = "/services",
        operation_id = "Check services",
        responses(
            (status = 200, description = "Check configured services")
        )
    )]
pub async fn services(State(services): State<Arc<Services>>) -> Result<Response, AppError> {
    Ok(api_response::success(StatusCode::OK, services.get_info())?)
}
