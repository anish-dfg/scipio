use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::app::api::v1::jobs::responses::{Job, JobsResponse};
use crate::app::errors::AppError;
use crate::app::state::Services;
use crate::services::storage::types::JobDetails;
use crate::services::storage::ExecOptsBuilder;

#[utoipa::path(
    get,
    path = "",
    operation_id = "Get jobs",
    responses(
        (status = 200, description = "Successfully fetched jobs"),
        (status = 401, description = "Unauthorized: invalid JWT"),
        (status = 403, description = "Forbidden: insufficient permissions (requires `fetch:cycle`)"),
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn fetch_jobs(State(ctx): State<Arc<Services>>) -> Result<Json<JobsResponse>, AppError> {
    let storage_layer = &ctx.storage_layer;
    let jobs: Vec<Job> = storage_layer
        .fetch_jobs(&mut ExecOptsBuilder::default().build()?)
        .await?
        .iter()
        .map(|job| {
            serde_json::from_value::<JobDetails>(job.details.clone()).map(|details| Job {
                id: job.id,
                label: job.label.clone(),
                description: job.description.clone(),
                created_at: job.created_at,
                updated_at: job.updated_at,
                project_cycle_id: job.project_cycle_id,
                status: job.status,
                details,
            })
        })
        .collect::<Result<Vec<Job>, _>>()?;

    let res = JobsResponse { jobs };
    Ok(Json(res))
}
