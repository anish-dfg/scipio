use anyhow::Result;
use axum::http::StatusCode;
use axum::response::Response;

use crate::app::api_response;
use crate::app::errors::AppError;

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
