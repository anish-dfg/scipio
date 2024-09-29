//! Controllers for the cycles API.

use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use uuid::Uuid;

use crate::app::api::v1::cycles::responses::CyclesResponse;
use crate::app::api_response;
use crate::app::errors::AppError;
use crate::app::state::Services;
use crate::services::storage::ExecOptsBuilder;

/// Fetch cycles
///
/// * `ctx`: The application context extracted as Axum state
#[utoipa::path(
    get,
    path = "",
    operation_id = "Get cycles",
    responses(
        (status = 200, description = "Successfully fetched cycles"),
        (status = 401, description = "Unauthorized: invalid JWT"),
        (status = 403, description = "Forbidden: insufficient permissions (requires `fetch:cycle`)"),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn fetch_cycles(State(ctx): State<Arc<Services>>) -> Result<Response, AppError> {
    let storage_layer = &ctx.storage_layer;
    let cycles = storage_layer.fetch_cycles(&mut ExecOptsBuilder::default().build()?).await?;
    let res = CyclesResponse { cycles };

    Ok(api_response::success(StatusCode::OK, res)?)
}

/// Delete a cycle
///
/// * `ctx`: The application context extracted as Axum state
/// * `id`: The ID of the cycle to delete
#[utoipa::path(
    delete,
    path = "/{id}",
    operation_id = "Get cycles",
    responses(
        (status = 204, description = "Successfully deleted cycle"),
        (status = 401, description = "Unauthorized: invalid JWT"),
        (status = 403, description = "Forbidden: insufficient permissions (requires `delete:cycle`)"),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn delete_cycle(
    State(ctx): State<Arc<Services>>,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let storage_layer = &ctx.storage_layer;
    storage_layer.delete_cycle(id, &mut ExecOptsBuilder::default().build()?).await?;
    Ok(api_response::no_content())
}
