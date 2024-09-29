use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::app::errors::AppError;
use crate::app::state::Services;
use crate::services::storage::entities::BasicStats;
use crate::services::storage::ExecOptsBuilder;

#[utoipa::path(
    get,
    path = "/{project_cycle_id}/basic",
    operation_id = "Get basic cycle stats",
    responses(
        (status = 200, description = "Successfully fetched jobs"),
        (status = 401, description = "Unauthorized: invalid JWT"),
        (status = 403, description = "Forbidden: insufficient permissions (requires `fetch:cycle`)"),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn fetch_basic_stats(
    State(ctx): State<Arc<Services>>,
    Path(project_cycle_id): Path<Uuid>,
) -> Result<Json<BasicStats>, AppError> {
    let storage_layer = &ctx.storage_layer;

    let stats = storage_layer
        .get_basic_stats(project_cycle_id, &mut ExecOptsBuilder::default().build()?)
        .await?;
    Ok(Json(stats))
}
