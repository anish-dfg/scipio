use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use chrono::Utc;
use tokio::task;

use super::ImportServices;
use crate::app::api::v1::data_imports::airtable::{import_task, ImportParams};
use crate::app::api::v1::data_imports::requests::ImportAirtableBase;
use crate::app::api::v1::data_imports::responses::AvailableBases;
use crate::app::api_response;
use crate::app::errors::AppError;
use crate::app::state::Services;
use crate::services::storage::jobs::CreateJobBuilder;
use crate::services::storage::types::{JobData, JobDetails, JobType};
use crate::services::storage::ExecOptsBuilder;

#[utoipa::path(
    get,
    path = "/airtable/available-bases",
    responses(
        (status = 200, description = "Successfully listed available bases"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_available_airtable_bases(
    State(ctx): State<Arc<Services>>,
) -> Result<Response, AppError> {
    let airtable = &ctx.airtable;

    let bases = airtable.list_available_bases().await?;
    let bases = AvailableBases { bases };

    Ok(api_response::success(StatusCode::OK, bases)?)
}

#[utoipa::path(
    post,
    path = "/airtable/base/{base_id}",
    request_body= ImportAirtableBase,
        responses(
            (status = 200, description = "Successfully imported a base"),
            (status = 400, description = "Malformed request body")
        )
    )
]
pub async fn import_airtable_base(
    State(services): State<ImportServices>,
    Path(base_id): Path<String>,
    Json(payload): Json<ImportAirtableBase>,
) -> Result<Response, AppError> {
    let storage_layer = &services.storage_layer;

    if !services.airtable.validate_schema(&base_id).await? {
        return Ok(api_response::error(
            StatusCode::BAD_REQUEST,
            "Invalid schema for airtable base",
        ));
    }

    let current_time = Utc::now();
    let time_only = current_time.format("%H:%M:%S").to_string();

    let data = CreateJobBuilder::default()
        .label("Import Airtable Base")
        .description(Some(format!("Import airtable base with id {base_id} ASDF")))
        .data(JobDetails {
            job_type: JobType::AirtableImportBase,
            error: None,
            data: JobData::AirtableImportBase { base_id: base_id.clone() },
        })
        .build()?;

    let job_id =
        storage_layer.create_job(None, data, &mut ExecOptsBuilder::default().build()?).await?;

    log::info!("Started import job {job_id} @ {time_only}");

    let params = ImportParams {
        name: payload.name,
        description: payload.description,
        job_id,
        base_id: base_id.clone(),
    };

    task::spawn(async move {
        let _ = import_task(&services, &params).await;
    });

    Ok(api_response::success(
        StatusCode::OK,
        serde_json::json!({
            "jobId": job_id
        }),
    )?)
}
