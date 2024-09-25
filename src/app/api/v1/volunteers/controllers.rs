use std::sync::Arc;

use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::app::api::v1::volunteers::responses::Volunteers;
use crate::app::context::Context;
use crate::app::errors::AppError;
use crate::services::storage::ExecOptsBuilder;

#[utoipa::path(
    get,
    path = "/{project_cycle_id}",
    operation_id = "Get cycle volunteers",
    responses(
        (status = 200, description = "Successfully fetched volunteers in cycle"),
        (status = 401, description = "Unauthorized: invalid JWT"),
        (status = 403, description = "Forbidden: insufficient permissions (requires `fetch:cycle`)"),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn fetch_volunteers_by_cycle(
    State(ctx): State<Arc<Context>>,
    Path(project_cycle_id): Path<Uuid>,
) -> Result<Json<Volunteers>, AppError> {
    let storage_layer = &ctx.storage_layer;
    let data = storage_layer
        .fetch_volunteers_by_cycle(project_cycle_id, &mut ExecOptsBuilder::default().build()?)
        .await?;

    Ok(Json(Volunteers { volunteers: data }))
}
