pub mod policies;

use anyhow::Result;
use policies::{EmailPolicy, PasswordPolicy};
use uuid::Uuid;

use super::ExportServices;
use crate::services::mail::{OnboardingEmailParams, OnboardingEmailParamsBuilder};
use crate::services::storage::entities::VolunteerDetails;
use crate::services::storage::volunteers::InsertVolunteerExportedToWorkspace;
use crate::services::storage::ExecOptsBuilder;
use crate::services::workspace::entities::{
    CreateWorkspaceUser, CreateWorkspaceUserBuilder, NameBuilder,
};

pub struct ExportParams {
    pub job_id: Uuid,
    pub principal: String,
    pub email_policy: EmailPolicy,
    pub password_policy: PasswordPolicy,
    pub volunteers: Vec<VolunteerDetails>,
}

struct ProcessedVolunteers {
    pub export_data: Vec<CreateWorkspaceUser>,
    pub pantheon_data: Vec<InsertVolunteerExportedToWorkspace>,
    pub onboarding_email_data: Vec<OnboardingEmailParams>,
}

fn process_volunteers(params: &ExportParams) -> Result<ProcessedVolunteers> {
    let mut pantheon_data =
        Vec::<InsertVolunteerExportedToWorkspace>::with_capacity(params.volunteers.len());

    let mut export_data = Vec::<CreateWorkspaceUser>::with_capacity(params.volunteers.len());

    let mut onboarding_email_data =
        Vec::<OnboardingEmailParams>::with_capacity(params.volunteers.len());

    for v in &params.volunteers {
        let primary_email = params.email_policy.build_email(&v.first_name, &v.last_name);
        let temporary_password = params.password_policy.generate_password();

        let workspace_user = CreateWorkspaceUserBuilder::default()
            .primary_email(primary_email.clone())
            .name(
                NameBuilder::default()
                    .given_name(v.first_name.clone())
                    .family_name(v.last_name.clone())
                    .build()
                    .unwrap(),
            )
            .password(temporary_password)
            .change_password_at_next_login(params.password_policy.change_password_at_next_login)
            .recovery_email(v.email.clone())
            // .recovery_phone(v.phone.clone())
            .build()?;

        onboarding_email_data.push(
            OnboardingEmailParamsBuilder::default()
                .first_name(workspace_user.name.given_name.clone())
                .last_name(workspace_user.name.family_name.clone())
                // .email(workspace_user.recovery_email.clone())
                .email("anish@developforgood.org")
                .workspace_email(workspace_user.primary_email.clone())
                .temporary_password(workspace_user.password.clone())
                .build()?,
        );

        export_data.push(workspace_user);

        pantheon_data.push(InsertVolunteerExportedToWorkspace {
            volunteer_id: v.volunteer_id,
            job_id: params.job_id,
            workspace_email: primary_email,
            org_unit: "/Programs/PantheonUsers".to_owned(),
        });
    }

    Ok(ProcessedVolunteers { export_data, pantheon_data, onboarding_email_data })
}

async fn export_volunteers_to_workspace(
    services: &ExportServices,
    principal: &str,
    export_data: Vec<CreateWorkspaceUser>,
) -> usize {
    let mut successfully_exported = 0usize;
    for user in export_data {
        let name = format!("{} {}", &user.name.given_name, &user.name.family_name);
        match services.workspace.create_user(principal, user).await {
            Ok(_) => {
                log::info!("Successfully exported user {} to workspace", name);
                successfully_exported += 1;
            }
            Err(e) => {
                log::error!("Failed to export user to workspace: {}", e);
                break;
            }
        }
    }

    successfully_exported
}

async fn save_exported_volunteers<'a>(
    services: &ExportServices,
    save_data: Vec<InsertVolunteerExportedToWorkspace>,
) -> Result<()> {
    services
        .storage_layer
        .batch_insert_volunteers_exported_to_workspace(
            save_data,
            &mut ExecOptsBuilder::default().build()?,
        )
        .await?;

    Ok(())
}

async fn send_onboarding_emails(
    services: &ExportServices,
    onboarding_data: Vec<OnboardingEmailParams>,
) -> Result<()> {
    for email in onboarding_data {
        services.mail.send_onboarding_email(email).await?;
    }
    Ok(())
}

pub async fn export_task(services: &ExportServices, params: ExportParams) -> Result<()> {
    let mut processed = process_volunteers(&params)?;

    let number_of_users_to_export = processed.export_data.len();
    let exported_count =
        export_volunteers_to_workspace(services, &params.principal, processed.export_data).await;

    if exported_count != number_of_users_to_export {
        log::error!(
            "Failed to export all users to workspace. Exported {} out of {}",
            exported_count,
            number_of_users_to_export
        );
        processed.pantheon_data.truncate(exported_count);
        processed.onboarding_email_data.truncate(exported_count);
    }

    match save_exported_volunteers(services, processed.pantheon_data).await {
        Ok(_) => match send_onboarding_emails(services, processed.onboarding_email_data).await {
            Ok(_) => {
                services
                    .storage_layer
                    .mark_job_complete(params.job_id, &mut ExecOptsBuilder::default().build()?)
                    .await?
            }
            Err(e) => {
                services
                    .storage_layer
                    .mark_job_errored(
                        params.job_id,
                        e.to_string(),
                        &mut ExecOptsBuilder::default().build()?,
                    )
                    .await?
            }
        },
        Err(e) => {
            services
                .storage_layer
                .mark_job_errored(
                    params.job_id,
                    e.to_string(),
                    &mut ExecOptsBuilder::default().build()?,
                )
                .await?
        }
    }

    Ok(())
}
