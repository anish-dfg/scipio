//! This module is responsible for the low level details of exporting users to Google Workspace.
//!
//! It contains the task that is spawned when a user requests to export volunteers to Google
//! Workspace.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_nats::Subscriber;
use chrono::Utc;
use futures::StreamExt;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::time::Duration;
use tokio::{task, time};
use uuid::Uuid;

use crate::app::api::v1::data_exports::requests::ExportUsersToWorkspaceRequest;
use crate::services::mail::{MailService, OnboardingEmailParamsBuilder};
use crate::services::storage::entities::VolunteerDetails;
use crate::services::storage::jobs::{CreateJobBuilder, UpdateJobStatus};
use crate::services::storage::types::{JobData, JobDetails, JobStatus, JobType};
use crate::services::storage::volunteers::InsertVolunteerExportedToWorkspace;
use crate::services::storage::{ExecOpts, ExecOptsBuilder, StorageService};
use crate::services::workspace::entities::{
    CreateWorkspaceUser, CreateWorkspaceUserBuilder, NameBuilder,
};
use crate::services::workspace::WorkspaceService;

pub struct ExportUsersToWorkspaceTaskParams {
    pub principal: String,
    pub request: ExportUsersToWorkspaceRequest,
    pub subscriber: Subscriber,
    pub job_id: Uuid,
    pub project_cycle_id: Uuid,
}

/// Builds an email address for a user in Google Workspace.
///
/// * `use_first_and_last_names`: Whether to use the first and last names of the user as part of
///   their email handle
/// * `add_unique_numeric_suffix`: Whether to add a unique (two digit) numeric suffix to the email
///   handle
/// * `separator`: The separator to use between the first and last names (if `use_first_and_last_names`
///   is set to `true`)
/// * `first_name`: The first name of the user
/// * `last_name`: The last name of the user
///
/// If there are spaces in the generated email, they will be removed. For example, if a volunteer
/// lists their first name as `Minh Uyen` and their last name as `Hoang`, the email will
/// concatenate `Minh` and `Uyen` into `minhuyen`.
fn build_email(
    use_first_and_last_names: bool,
    add_unique_numeric_suffix: bool,
    separator: Option<String>,
    first_name: &str,
    last_name: &str,
) -> String {
    let mut base = if use_first_and_last_names {
        format!("{}{}{}", first_name, separator.unwrap_or("".to_string()), last_name)
    } else {
        first_name.to_owned()
    };

    if add_unique_numeric_suffix {
        let mut rng = rand::thread_rng();
        let suffix = rng.gen_range(10..100);

        base.push_str(&suffix.to_string());
    }

    let mut cleaned = base.chars().filter(|c| c.is_alphanumeric()).collect::<String>();

    cleaned.push_str("@developforgood.org");
    cleaned
}

/// Generates a random password between 8 and 64 characters for a user in Google Workspace.
///
/// * `len`: The length of the password to generate (must be between 8 and 64 characters). If it
///   isn't, the function will default to 8 characters.
fn generate_password(len: u8) -> String {
    if !(8..=64).contains(&len) {
        log::warn!(
            "Password length must be between 8 and 64 characters. Defaulting to 8 characters."
        );
    }
    match len {
        // minimum, and default, is 8. max is 64
        0..=7 | 65.. => rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect::<String>(),
        8..=64 => rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len as usize)
            .map(char::from)
            .collect::<String>(),
    }
}

/// The result of processing volunteers for export to Google Workspace.
///
/// * `processed_volunteers`: Data to create users in Google Workspace
/// * `pantheon_user_data`: Data to insert into the pantheon database
/// * `id_map`: A map of the generated email addresses to the Pantheon IDs of the volunteers
struct ProcessVolunteersResult {
    processed_volunteers: Vec<CreateWorkspaceUser>,
    pantheon_user_data: Vec<InsertVolunteerExportedToWorkspace>,
    id_map: HashMap<String, Uuid>,
}

/// Processes volunteers for export to Google Workspace.
///
/// * `params`: Data about the export request
/// * `volunteers`: Data to process
fn process_volunteers(
    params: &ExportUsersToWorkspaceTaskParams,
    volunteers: Vec<VolunteerDetails>,
) -> Result<ProcessVolunteersResult> {
    // Maps the _generated_ @developforgood.org email address of a volunteer to their Pantheon ID.
    let mut id_map = HashMap::<String, Uuid>::with_capacity(volunteers.len());

    // Pantheon export data refers to the data that will be stored in the pantheon database. We
    // will insert records for each exported volunteer.
    let mut pantheon_export_data =
        Vec::<InsertVolunteerExportedToWorkspace>::with_capacity(volunteers.len());

    let mut processed = Vec::with_capacity(volunteers.len());

    for v in volunteers.iter() {
        let primary_email = build_email(
            params.request.use_first_and_last_name,
            params.request.add_unique_numeric_suffix,
            params.request.separator.clone(),
            &v.first_name.to_lowercase(),
            &v.last_name.to_lowercase(),
        );

        let password = generate_password(params.request.generated_password_length);

        // Associate the generated email address with the volunteer's ID in Pantheon.
        id_map.insert(primary_email.clone(), v.volunteer_id);

        let workspace_user = CreateWorkspaceUserBuilder::default()
            .primary_email(primary_email.clone())
            .name(
                NameBuilder::default()
                    .given_name(v.first_name.clone())
                    .family_name(v.last_name.clone())
                    .build()
                    .unwrap(),
            )
            .password(password)
            .change_password_at_next_login(params.request.change_password_at_next_login)
            .recovery_email(v.email.clone())
            // .recovery_phone(v.phone.clone())
            .build()?;

        pantheon_export_data.push(InsertVolunteerExportedToWorkspace {
            volunteer_id: v.volunteer_id,
            job_id: params.job_id,
            workspace_email: primary_email,
            org_unit: "/Programs/PantheonUsers".to_owned(),
        });

        processed.push(workspace_user);
    }

    Ok(ProcessVolunteersResult {
        processed_volunteers: processed,
        pantheon_user_data: pantheon_export_data,
        id_map,
    })
}

/// Exports users to Google Workspace.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `mail`: Handle to the email client
/// * `principal`: The principal that is making the request
/// * `result`: The result of processing the volunteers
/// * `successfully_exported_users`: A list to keep track of users that are successfully exported
///   in this function
///
/// At its core, this function creates users in Google Workspace and sends them an onboarding
/// email. If this was successful, it will mark the users as exported in the pantheon database.
async fn export_users_to_workspace(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    mail: Arc<dyn MailService>,
    principal: &str,
    result: ProcessVolunteersResult,
    successfully_exported_users: &mut Vec<(Uuid, String)>,
) -> Result<()> {
    // NOTE: On the UI, there is an undo button that appears on the toast that pops up after the
    // user clicks the "Export" button. This button will send a signal to cancel the export job and
    // we want to have 7 seconds of leeway to kind of 'wait' so that we don't do any work that's
    // just going to be undone. If they change their minds later, then we will undo the work that
    // we've done, but this is a small optimization to prevent unnecessary work.

    log::info!("waiting for potential immediate cancellation");
    time::sleep(Duration::from_secs(7)).await;

    let ProcessVolunteersResult { processed_volunteers, pantheon_user_data, id_map } = result;

    for volunteer in processed_volunteers {
        let email = volunteer.recovery_email.clone();
        let primary_email = volunteer.primary_email.clone();

        // Try to create the user in Google Workspace
        workspace.create_user(principal, volunteer.clone()).await?;
        if let Some(id) = id_map.get(&primary_email) {
            successfully_exported_users.push((*id, primary_email.clone()));
        }

        let now = Utc::now().timestamp() as u64;

        let email_params = OnboardingEmailParamsBuilder::default()
            .first_name(volunteer.name.given_name)
            .last_name(volunteer.name.family_name)
            // .email(email.clone())
            .email("anish@developforgood.org")
            .workspace_email(primary_email.clone())
            .temporary_password(volunteer.password)
            .send_at(now + 60)
            .build()?;

        mail.send_onboarding_email(email_params).await?;

        log::info!("exported user: {} -> {}\nSLEEPING FOR 15 SECONDS", email, primary_email);

        // If you're trying to test the undo functionality, you can uncomment the sleep since it
        // makes it easier to cancel before the job finishes, since you're probably using this in
        // tandem with NoopWorkspaceClient.
        // time::sleep(Duration::from_secs(15)).await;
    }

    // Mark the volunteers as exported to Google Workspace (this only happens if every single
    // volunteer was exported successfully)

    if !pantheon_user_data.is_empty() {
        storage_layer
            .batch_insert_volunteers_exported_to_workspace(
                pantheon_user_data,
                &mut ExecOptsBuilder::default().build()?,
            )
            .await?;
    }

    Ok(())
}

/// Undoes a partial export of users to Google Workspace.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `principal`: The principal that is making the request
/// * `successfully_exported_users`: The list of users that were successfully exported
///
/// This function will greedily attempt to delete every user in `successfully_exported_users` from
/// both Google Workspace and the pantheon database. If a user cannot be deleted from Google
/// Workspace, they will not be removed from the pantheon database either, and an error will be
/// logged.
async fn undo_partial_export(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    principal: &str,
    successfully_exported_users: Vec<(Uuid, String)>,
) -> Result<Vec<(Uuid, String)>> {
    // This is where the users are removed from workspace (and pantheon's database)

    // Track the successfully deleted users (in workspace)
    let mut successfully_deleted_users =
        Vec::<(Uuid, String)>::with_capacity(successfully_exported_users.len());

    // Track the failed deletions (in workspace)
    let mut failed_deletions = Vec::<(Uuid, String)>::new();

    // Delete the users from Google Workspace
    for (id, workspace_email) in &successfully_exported_users {
        // NOTE: We don't propagate this error because we want to attempt to delete every possible
        // user requested.
        log::info!("SLEEPING FOR 5 SECONDS");
        time::sleep(Duration::from_secs(5)).await;
        log::info!("deleting user: {workspace_email}");
        match workspace.delete_user(principal, workspace_email).await {
            Ok(_) => successfully_deleted_users.push((*id, workspace_email.to_owned())),
            Err(e) => {
                log::error!("error deleting user: {e}");
                failed_deletions.push((*id, workspace_email.to_owned()));
            }
        }
    }

    // Remove the users from the pantheon database
    storage_layer
        .batch_remove_volunteers_exported_to_workspace(
            successfully_deleted_users.iter().map(|(id, _)| *id).collect(),
            &mut ExecOptsBuilder::default().build()?,
        )
        .await?;

    Ok(failed_deletions)
}

/// Parameters for the undo export task.
///
/// * `job_id`: The ID of the job that is being undone
/// * `project_cycle_id`: The ID of the project cycle that the job is associated with
/// * `principal`: The principal that is making the request
/// * `successfully_exported_users`: The users we are attempting to delete
struct UndoExportTaskParams {
    job_id: Uuid,
    project_cycle_id: Uuid,
    principal: String,
    successfully_exported_users: Vec<(Uuid, String)>,
}

/// The post-cancellation undo export task.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `undo_job_id`: The ID of the job that is undoing the export
/// * `params`: The parameters for the undo task
async fn post_cancellation_undo_partial_export_task(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    undo_job_id: Uuid,
    params: UndoExportTaskParams,
) -> Result<()> {
    // NOTE: undo_partial_export_task will attempt to delete all users possible. Failed
    // deletes will be returned as a Vec<Uuid> where each Uuid represents a volunteer
    // in Pantheon. It will be empty if all deletions were successful.
    let principal = params.principal.clone();
    let res = undo_partial_export(
        storage_layer.clone(),
        workspace.clone(),
        &principal,
        params.successfully_exported_users,
    )
    .await;

    let (status, error) = if res.is_ok() {
        (JobStatus::Complete, None)
    } else {
        (JobStatus::Error, Some(format!("failed to undo partial export: {res:?}")))
    };

    storage_layer
        .update_job_status(
            undo_job_id,
            UpdateJobStatus { status, error },
            &mut ExecOpts { tx: None },
        )
        .await?;

    Ok(())
}

/// Handles the post-cancellation of an export job.
///
/// This starts an async job to handle the cancellation to undo the export
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `params`: The parameters for the post-cancellation task
async fn handle_post_cancellation_undo_export(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    params: UndoExportTaskParams,
) -> Result<()> {
    // Create a job to undo the partial export
    let current_time = Utc::now();
    let time_only = current_time.format("%H:%M:%S").to_string();
    let undo_job_id = storage_layer
        .create_job(
            Some(params.project_cycle_id),
            CreateJobBuilder::default()
                .label(format!("Undo Partial Export @ {time_only}"))
                .description(Some("Undo partial export of users to Google Workspace".to_owned()))
                .data(JobDetails {
                    job_type: JobType::UndoWorkspaceExport,
                    error: None,
                    data: JobData::UndoWorkspaceExport {
                        volunteers: params.successfully_exported_users.clone(),
                    },
                })
                .build()?,
            &mut ExecOptsBuilder::default().build()?,
        )
        .await?;

    // Mark the current export job as canceled
    storage_layer
        .update_job_status(
            params.job_id,
            UpdateJobStatus { status: JobStatus::Cancelled, error: None },
            &mut ExecOpts { tx: None },
        )
        .await?;

    log::info!(
        "successfully resolved export job (job status: cancelled). undo job id: {undo_job_id}"
    );

    // Spawn a nonblocking task to handle the undo
    task::spawn(async move {
        let _ = post_cancellation_undo_partial_export_task(
            storage_layer.clone(),
            workspace.clone(),
            undo_job_id,
            params,
        )
        .await;
    });
    Ok(())
}

/// Parameters for the post-cancellation handler.
///
/// * `job_id`: The ID of the job that was canceled
/// * `project_cycle_id`: The ID of the project cycle that the job is associated with
/// * `principal`: The principal that is making the request
struct HandlePostCancellationParams {
    job_id: Uuid,
    project_cycle_id: Uuid,
    principal: String,
}

/// Wraps the handling of the post-cancellation of an export job.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `params`: The parameters for the post-cancellation handler
/// * `successfully_exported_users`: The users that were successfully exported
async fn handle_post_cancellation(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    params: HandlePostCancellationParams,
    successfully_exported_users: Vec<(Uuid, String)>,
) -> Result<()> {
    let undo_params = UndoExportTaskParams {
        job_id: params.job_id,
        project_cycle_id: params.project_cycle_id,
        principal: params.principal.clone(),
        successfully_exported_users: successfully_exported_users.clone(),
    };
    handle_post_cancellation_undo_export(storage_layer, workspace, undo_params).await?;
    Ok(())
}

/// Parameters for the post-export handler.
///
/// * `export_result`: The result of the export
/// * `job_id`: The ID of the job that handled the export
/// * `project_cycle_id`: The ID of the project cycle that the job is associated with
/// * `principal`: The principal that is making the request
/// * `successfully_exported_users`: The users that were successfully exported
struct HandlePostExportParams {
    export_result: Result<()>,
    job_id: Uuid,
    project_cycle_id: Uuid,
    principal: String,
    successfully_exported_users: Vec<(Uuid, String)>,
}

/// The task that handles the undo of an export job if the export job was not a complete success.
/// (if all users were not exported successfully).
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `principal`: The principal that is making the request
/// * `undo_job_id`: The ID of the job that is undoing the export
/// * `successfully_exported_users`: The users that were successfully exported
async fn post_export_undo_task(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    principal: &str,
    undo_job_id: Uuid,
    successfully_exported_users: Vec<(Uuid, String)>,
) -> Result<()> {
    let failed_deletions = undo_partial_export(
        storage_layer.clone(),
        workspace.clone(),
        principal,
        successfully_exported_users,
    )
    .await?;

    let (status, error) = if failed_deletions.is_empty() {
        (JobStatus::Complete, None)
    } else {
        log::error!("failed to delete {} users: {failed_deletions:?}", failed_deletions.len());
        (
            JobStatus::Error,
            Some(format!(
                "failed to delete {} users: {failed_deletions:?}",
                failed_deletions.len()
            )),
        )
    };

    storage_layer
        .update_job_status(
            undo_job_id,
            UpdateJobStatus { status, error },
            &mut ExecOpts { tx: None },
        )
        .await?;

    Ok(())
}

/// Handles the post-export of an export job.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `params`: Parameters for the post-export handler
async fn handle_post_export(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    params: HandlePostExportParams,
) -> Result<()> {
    // [F2-2]: Based on the export result, update the job status
    let res = params.export_result;
    let (status, error) = if res.is_ok() {
        (JobStatus::Complete, None)
    } else {
        (
            JobStatus::Error,
            Some(format!("unrecoverable error exporting all users to Google Workspace: {res:?}")),
        )
    };

    storage_layer
        .update_job_status(
            params.job_id,
            UpdateJobStatus { status, error },
            &mut ExecOpts { tx: None },
        )
        .await?;

    // [F2-3]: If there was an error, create a job to document the undo process
    if JobStatus::Error == status && !params.successfully_exported_users.is_empty() {
        let current_time = Utc::now();
        let time_only = current_time.format("%H:%M:%S").to_string();
        let undo_job_id = storage_layer
            .create_job(
                Some(params.project_cycle_id),
                CreateJobBuilder::default()
                    .label(format!("Undo Partial Export @ {time_only}"))
                    .description(Some(
                        "Undo partial export of users to Google Workspace".to_owned(),
                    ))
                    .data(JobDetails {
                        job_type: JobType::UndoWorkspaceExport,
                        error: None,
                        data: JobData::UndoWorkspaceExport {
                            volunteers: params.successfully_exported_users.clone(),
                        },
                    })
                    .build()?,
                &mut ExecOptsBuilder::default().build()?,
            )
            .await?;

        // [F2-4]: Spawn a nonblocking task to handle the undo
        task::spawn(async move {
            let _ = post_export_undo_task(
                storage_layer.clone(),
                workspace.clone(),
                &params.principal,
                undo_job_id,
                params.successfully_exported_users,
            )
            .await;
        });
    }

    Ok(())
}

/// The export task that is spawned when a user requests to export volunteers to Google Workspace.
///
/// * `storage_layer`: Handle to the storage layer
/// * `workspace`: Handle to the workspace client
/// * `mail`: Handle to the mail client
/// * `params`: Parameters for the export task
pub async fn export_task(
    storage_layer: Arc<dyn StorageService>,
    workspace: Arc<dyn WorkspaceService>,
    mail: Arc<dyn MailService>,
    mut params: ExportUsersToWorkspaceTaskParams,
) -> Result<()> {
    let process_result = process_volunteers(&params, params.request.volunteers.clone())?;

    // NOTE: This maps volunteer IDs in pantheon to the generated email addresses in Google Workspace.
    // The emails are NOT the same as the ones in the email field in the volunteers relation in
    // pantheon's database. This needs to be here because the way we handle multiple futures in the
    // tokio::select! macro necessitates that multiple handlers have this data in order to
    // facilitate the undo process if one is necessary.
    let mut successfully_exported_users =
        Vec::<(Uuid, String)>::with_capacity(process_result.processed_volunteers.len());

    // Race the following futures:
    //
    // 1 Await a cancellation signal:
    //  - If received, create a job to undo the partial export, and then spawn an async task to
    //    handle the deletion
    // 2. Export all users to Google Workspace
    //  - If successful, mark the job as complete
    //  - If failed, mark the job as errored
    // 3. Timeout after 10 minutes
    //
    tokio::select! {
        // Future 1
        _ = params.subscriber.next() => {
            // NOTE: At this point, we have received a signal to cancel this export job.
            log::info!("request cancelled...attempting to undo partial export");
            let post_cancel_params = HandlePostCancellationParams {
                job_id: params.job_id,
                project_cycle_id: params.project_cycle_id,
                principal: params.principal.clone(),
            };
            log::info!("successfully_exported_users: {successfully_exported_users:?}");
            handle_post_cancellation(storage_layer.clone(), workspace.clone(), post_cancel_params, successfully_exported_users).await?;
        },

        // Future 2
        res = export_users_to_workspace(storage_layer.clone(), workspace.clone(), mail.clone(), &params.principal, process_result, &mut successfully_exported_users) => {
            // NOTE: At this point, we have completed out attempt to export all users to Google Workspace.

            // [F2-1]: Call a post-export hook with the right parameters
            let post_export_params = HandlePostExportParams {
                export_result: res,
                job_id: params.job_id,
                project_cycle_id: params.project_cycle_id,
                principal: params.principal.clone(),
                successfully_exported_users,
            };

            handle_post_export(storage_layer, workspace, post_export_params).await?;
        },

        // Future 3
        () = time::sleep(Duration::from_secs(1200)) => {
            // NOTE: A request times out after 10 minutes. An export task really shouldn't take
            // this long.
            log::warn!("request timed out");
        }
    };

    Ok(())
}
