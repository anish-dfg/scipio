//! Controllers for the data exports API.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::{Extension, Json};
use chrono::Utc;
use tokio::task;
use uuid::Uuid;

use crate::app::api::v1::data_exports::export_users_to_workspace::{
    export_task, ExportUsersToWorkspaceTaskParams,
};
use crate::app::api::v1::data_exports::requests::ExportUsersToWorkspaceRequest;
use crate::app::api::v1::data_exports::responses::ExportUsersToWorkspaceResponse;
use crate::app::api_response;
use crate::app::context::Context;
use crate::app::errors::AppError;
use crate::services::auth::AuthData;
use crate::services::storage::jobs::CreateJobBuilder;
use crate::services::storage::types::{ExportDesination, JobData, JobDetails, JobType};
use crate::services::storage::ExecOptsBuilder;

/// Start a job to export users to Google Workspace.
///
/// * `ctx`:  The application context
/// * `project_cycle_id`: The ID of the project cycle
/// * `auth`: Auth data about the user
/// * `request`: The request data
///
/// This endpoint starts a job, records it in the database, and returns immediately. The task it
/// spawns does not block, and it can be cancelled by sending a message to the NATS server by
/// publishing to the topic `pantheon.export.cancel.{job_id}` where `job_id` is the ID of the job in
/// the database. Data about the job can be retrieved by using the Jobs API.
#[utoipa::path(
    post,
    path = "/{project_cycle_id}/workspace",
    responses(
        (status = 200, description = "Successfully started job to export users to Google Workspace"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    params(
        ("Authorization" = String, Header, description = "JWT. NOTE: Prefix with Bearer")
    ),
)]
pub async fn export_users_to_workspace(
    State(ctx): State<Arc<Context>>,
    Path(project_cycle_id): Path<Uuid>,
    Extension(auth): Extension<AuthData>,
    Json(request): Json<ExportUsersToWorkspaceRequest>,
) -> Result<Response, AppError> {
    let storage_layer = ctx.storage_layer.clone();
    let workspace = ctx.workspace.clone();
    let nats = ctx.nats.clone();
    let mail = ctx.mail.clone();

    let current_time = Utc::now();
    let time_only = current_time.format("%H:%M:%S").to_string();

    let data = CreateJobBuilder::default()
        .label(format!("Export Users @ {time_only}"))
        .description(Some("Export users to Google Workspace".to_owned()))
        .data(JobDetails {
            job_type: JobType::AirtableExportUsers,
            error: None,
            data: JobData::AirtableExportUsers {
                export_destination: ExportDesination::GoogleWorkspace,
            },
        })
        .build()?;

    let job_id = storage_layer
        .create_job(Some(project_cycle_id), data, &mut ExecOptsBuilder::default().build()?)
        .await?;

    let subject = format!("pantheon.export.cancel.{}", job_id);
    log::info!("{job_id}");

    let mut subscriber = nats.subscribe(subject).await?;
    subscriber.unsubscribe_after(1).await?;

    let principal = auth.email()?;

    let params = ExportUsersToWorkspaceTaskParams {
        subscriber,
        request,
        principal,
        job_id,
        project_cycle_id,
    };

    task::spawn(async move {
        let _ = export_task(storage_layer.clone(), workspace.clone(), mail, params).await;
    });

    Ok(api_response::success(StatusCode::OK, ExportUsersToWorkspaceResponse { job_id })?)
}
