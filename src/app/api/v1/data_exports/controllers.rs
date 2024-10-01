//! Controllers for the data exports API.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::{Extension, Json};
use chrono::Utc;
use tokio::task;
use uuid::Uuid;

use super::workspace::policies::{EmailPolicy, PasswordPolicy};
use super::workspace::{export_task, ExportParams};
use super::ExportServices;
use crate::app::api::v1::data_exports::requests::ExportUsersToWorkspaceRequest;
use crate::app::api::v1::data_exports::responses::ExportUsersToWorkspaceResponse;
use crate::app::api_response;
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
/// spawns does not block.
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
    State(services): State<ExportServices>,
    Path(project_cycle_id): Path<Uuid>,
    Extension(auth): Extension<AuthData>,
    Json(request): Json<ExportUsersToWorkspaceRequest>,
) -> Result<Response, AppError> {
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

    let job_id = services
        .storage_layer
        .create_job(Some(project_cycle_id), data, &mut ExecOptsBuilder::default().build()?)
        .await?;

    let email_policy = EmailPolicy::from(&request);
    let password_policy = PasswordPolicy::from(&request);

    let already_exported = services
        .storage_layer
        .fetch_exported_volunteer_details_by_project_cycle(
            project_cycle_id,
            &mut ExecOptsBuilder::default().build()?,
        )
        .await?
        .iter()
        .map(|v| v.volunteer_id)
        .collect::<Vec<Uuid>>();

    let volunteers = if request.skip_users_on_conflict {
        log::info!("Skipping users that have already been exported");
        request
            .volunteers
            .into_iter()
            .filter(|v| !already_exported.contains(&v.volunteer_id))
            .collect()
    } else {
        for v in &request.volunteers {
            if already_exported.contains(&v.volunteer_id) {
                log::error!("One or more users have already been exported");
                return Ok(api_response::error(
                    StatusCode::BAD_REQUEST,
                    "One or more users have already been exported",
                ));
            }
        }
        request.volunteers
    };

    let params = ExportParams {
        job_id,
        email_policy,
        password_policy,
        principal: auth.email()?,
        volunteers,
    };

    task::spawn(async move {
        let _ = export_task(&services, params).await;
    });

    Ok(api_response::success(StatusCode::OK, ExportUsersToWorkspaceResponse { job_id })?)
}
