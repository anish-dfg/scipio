//! This module contains the definition of the `QueryVolunteers` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::{Context, Result};
use async_trait::async_trait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use super::entities::{ExportedVolunteerDetails, VolunteerDetails};
use super::exec_with_tx;
use super::types::{AgeRange, Ethnicity, Fli, Gender, Lgbt, StudentStage, VolunteerHearAbout};
use crate::services::storage::{Acquire, ExecOpts, PgBackend};

/// Create a new volunteer.
///
/// * `first_name`: The first name of the volunteer
/// * `last_name`: The last name of the volunteer
/// * `email`: The email of the volunteer (their personal email, not their workspace email)
/// * `phone`: The phone number of the volunteer
/// * `offer_letter_signature`: Whether the volunteer has signed the offer letter
/// * `volunteer_gender`: The gneder of the volunteer
/// * `volunteer_ethnicity`: The ethnicity of the volunteer
/// * `volunteer_age_range`: The age range of the volunteer
#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateVolunteer {
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub phone: Option<String>,
    #[builder(default = "Gender::PreferNotToSay")]
    pub volunteer_gender: Gender,
    #[builder(default = "vec![Ethnicity::PreferNotToSay]")]
    pub volunteer_ethnicity: Vec<Ethnicity>,
    #[builder(default = "AgeRange::R18_24")]
    pub volunteer_age_range: AgeRange,
    #[builder(default = "vec![]")]
    pub university: Vec<String>,
    #[serde(rename = "LGBT")]
    pub lgbt: Lgbt,
    pub country: String,
    #[builder(setter(into), default = "None")]
    #[serde(rename = "State")]
    pub us_state: Option<String>,
    #[builder(default = "vec![Fli::PreferNotToSay]")]
    #[serde(rename = "FLI")]
    pub fli: Vec<Fli>,
    pub student_stage: StudentStage,
    pub majors: Vec<String>,
    pub minors: Vec<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
}

/// Edit a volunteer.
///
/// * `email`: The new email of the volunteer (if they change their personal email)
/// * `phone`: The new phone number of the volunteer
/// * `offer_letter_signature`: The new status of the volunteer's offer letter signature
#[derive(Builder)]
pub struct EditVolunteer {
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub phone: Option<String>,
}

/// Record a volunteer as exported to a workspace.
///
/// * `volunteer_id`: The ID of the volunteer
/// * `job_id`: The ID of the job that exported the volunteer
/// * `workspace_email`: The workspace email the volunteer has been issued
/// * `org_unit`: Which Develop for Good Organizational Unit the volunteer has been exported to
///   (usually "/Programs/PantheonUsers")
#[derive(Builder, Clone)]
pub struct InsertVolunteerExportedToWorkspace {
    pub volunteer_id: Uuid,
    pub job_id: Uuid,
    #[builder(setter(into))]
    pub workspace_email: String,
    #[builder(setter(into))]
    pub org_unit: String,
}

/// A trait for querying data about volunteers.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryVolunteers<DB: Database> {
    /// Create a new volunteer.
    ///
    /// * `project_cycle_id`: The ID of the project cycle the volunteer is associated with
    /// * `data`: Data required to create the volunteer
    /// * `exec_opts`: Execution options for the query
    async fn create_volunteer(
        &self,
        project_cycle_id: Uuid,
        data: CreateVolunteer,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Uuid> {
        unimplemented!()
    }

    /// Batch create volunteers.
    ///
    /// * `project_cycle_id`: The ID of the project cycle all of the volunteers are associated with
    /// * `data`: Data required to create the volunteers
    /// * `exec_opts`: Execution options for the query
    async fn batch_create_volunteers(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateVolunteer>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Vec<(String, Uuid)>> {
        unimplemented!()
    }

    /// Fetch all volunteers.
    ///
    /// * `exec_opts`: Execution options for the query
    ///
    /// This function should be deprecated in favor of a version that takes filtering and pagination options.
    // TODO: Add a new function that fetches volunteers with filtering and pagination options
    async fn fetch_volunteers(
        &self,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Vec<VolunteerDetails>> {
        unimplemented!()
    }

    /// Fetch all volunteers associated with a project cycle.
    ///
    /// * `project_cycle_id`: The ID of the project cycle to fetch volunteers for
    /// * `exec_opts`: Execution options for the query
    ///
    /// This function should be deprecated in favor of a version of `fetch_volunteers` that takes filtering and
    /// pagination options.
    // TODO: Deprecate this function after a generic version of `fetch_volunteers` with filtering
    // and pagination options is implemented
    async fn fetch_volunteers_by_cycle(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Vec<VolunteerDetails>> {
        unimplemented!()
    }

    /// Fetch a volunteer by ID.
    ///
    /// * `id`: The ID of the volunteer to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_volunteer_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Option<VolunteerDetails>> {
        unimplemented!()
    }

    /// Fetch a volunteer by email.
    ///
    /// * `email`: The email of the volunteer to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_volunteer_by_email(
        &self,
        email: &str,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Option<VolunteerDetails>> {
        unimplemented!()
    }

    /// Edit a volunteer.
    ///
    /// * `id`: The ID of the volunteer to edit
    /// * `data`: Data required to edit the volunteer
    /// * `exec_opts`: Execution options for the query
    async fn edit_volunteer(
        &self,
        id: Uuid,
        data: EditVolunteer,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Delete a volunteer by ID.
    ///
    /// * `id`: The ID of the volunteer to delete
    /// * `exec_opts`: Execution options for the query
    async fn delete_volunteer(&self, id: Uuid, exec_opts: &mut ExecOpts<DB>) -> Result<()> {
        unimplemented!()
    }

    /// Batch link volunteers to nonprofits.
    ///
    /// * `project_cycle_id`: The project cycle ID that the volunteers and mentors are associated
    ///   with
    /// * `linkage`: The linkage, represented as a vector of 2-tuples mapping volunteer IDs to
    ///   nonprofit IDs
    /// * `exec_opts`: Execution options for the query
    async fn batch_link_volunteers_to_nonprofits(
        &self,
        project_cycle_id: Uuid,
        linkage: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Batch link volunteers to mentors.
    ///
    /// * `project_cycle_id`: The project cycle ID that the volunteers and mentors are associated
    ///   with
    /// * `linkage`: The linkage, represented as a vector of 2-tuples mapping volunteer IDs to
    ///   mentor IDs
    /// * `exec_opts`: Execution options for the query
    async fn batch_link_volunteers_to_mentors(
        &self,
        project_cycle_id: Uuid,
        linkage: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Batch record volunteers as exported to a workspace.
    ///
    /// * `data`: Data required to record the volunteers as exported to a workspace
    /// * `exec_opts`: Execution options for the query
    async fn batch_insert_volunteers_exported_to_workspace(
        &self,
        data: Vec<InsertVolunteerExportedToWorkspace>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Batch undo the recording of volunteers as exported to a workspace.
    ///
    /// * `data`: Data required to undo the recording of the volunteers as exported to a workspace
    /// * `exec_opts`: Execution options for the query
    async fn batch_remove_volunteers_exported_to_workspace(
        &self,
        data: Vec<Uuid>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn fetch_exported_volunteer_details_by_project_cycle(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Vec<ExportedVolunteerDetails>> {
        unimplemented!()
    }
}

#[async_trait]
impl QueryVolunteers<Postgres> for PgBackend {
    async fn create_volunteer(
        &self,
        project_cycle_id: Uuid,
        data: CreateVolunteer,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Uuid> {
        async fn exec(
            project_cycle_id: Uuid,
            data: CreateVolunteer,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Uuid> {
            let query = include_str!("queries/volunteers/create_volunteer.sql");

            let id = sqlx::query_scalar::<_, Uuid>(query)
                .bind(project_cycle_id)
                .bind(data.first_name)
                .bind(data.last_name)
                .bind(data.email)
                .bind(data.phone)
                .bind(data.volunteer_gender)
                .bind(data.volunteer_ethnicity)
                .bind(data.volunteer_age_range)
                .bind(data.university)
                .bind(data.lgbt)
                .bind(data.country)
                .bind(data.us_state)
                .bind(data.fli)
                .bind(data.student_stage)
                .bind(data.majors)
                .bind(data.minors)
                .bind(data.hear_about)
                .fetch_one(&mut **tx)
                .await?;

            Ok(id)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn batch_create_volunteers(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateVolunteer>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<(String, Uuid)>> {
        async fn exec(
            project_cycle_id: Uuid,
            data: Vec<CreateVolunteer>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Vec<(String, Uuid)>> {
            let fragment = include_str!("queries/volunteers/create_volunteers.fragment.sql");
            let data = QueryBuilder::<Postgres>::new(fragment)
                .push_values(data, |mut b, v| {
                    b.push_bind(project_cycle_id)
                        .push_bind(v.first_name)
                        .push_bind(v.last_name)
                        .push_bind(v.email)
                        .push_bind(v.phone)
                        .push_bind(v.volunteer_gender)
                        .push_bind(v.volunteer_ethnicity)
                        .push_bind(v.volunteer_age_range)
                        .push_bind(v.university)
                        .push_bind(v.lgbt)
                        .push_bind(v.country)
                        .push_bind(v.us_state)
                        .push_bind(v.fli)
                        .push_bind(v.student_stage)
                        .push_bind(v.majors)
                        .push_bind(v.minors)
                        .push_bind(v.hear_about);
                })
                .push(" returning email, id")
                .build_query_as::<(String, Uuid)>()
                .fetch_all(&mut **tx)
                .await?;
            Ok(data)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn fetch_volunteer_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<VolunteerDetails>> {
        async fn exec(
            id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<VolunteerDetails>> {
            let query = include_str!("queries/volunteers/fetch_volunteer_by_id.sql");
            let volunteer = sqlx::query_as::<_, VolunteerDetails>(query)
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .context("error fetching volunteer by id")?;
            Ok(volunteer)
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn fetch_volunteer_by_email(
        &self,
        email: &str,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<VolunteerDetails>> {
        async fn exec<'b>(
            email: &'b str,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<VolunteerDetails>> {
            let query = include_str!("queries/volunteers/fetch_volunteer_by_email.sql");
            let volunteer = sqlx::query_as::<_, VolunteerDetails>(query)
                .bind(email)
                .fetch_optional(&mut **tx)
                .await
                .context("error fetching volunteer by id")?;
            Ok(volunteer)
        }

        exec_with_tx!(self, exec_opts, exec, email)
    }

    async fn fetch_volunteers(
        &self,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<VolunteerDetails>> {
        async fn exec(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<VolunteerDetails>> {
            let query = include_str!("queries/volunteers/fetch_volunteers.sql");
            let volunteers = sqlx::query_as::<_, VolunteerDetails>(query)
                .fetch_all(&mut **tx)
                .await
                .context("error fetching volunteers")?;
            Ok(volunteers)
        }

        exec_with_tx!(self, exec_opts, exec)
    }

    async fn fetch_volunteers_by_cycle(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<VolunteerDetails>> {
        async fn exec(
            project_cycle_id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Vec<VolunteerDetails>> {
            let query = include_str!("queries/volunteers/fetch_volunteers_by_cycle.sql");
            let volunteers = sqlx::query_as::<_, VolunteerDetails>(query)
                .bind(project_cycle_id)
                .fetch_all(&mut **tx)
                .await
                .context("error fetching volunteers by cycle")?;
            Ok(volunteers)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id)
    }

    async fn edit_volunteer(
        &self,
        id: Uuid,
        data: EditVolunteer,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            id: Uuid,
            data: EditVolunteer,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let query = include_str!("queries/volunteers/edit_volunteer.sql");
            sqlx::query(query)
                .bind(id)
                .bind(data.email)
                .bind(data.phone)
                .execute(&mut **tx)
                .await
                .context("error editing volunteer")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id, data)
    }

    async fn delete_volunteer(&self, id: Uuid, exec_opts: &mut ExecOpts<Postgres>) -> Result<()> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/volunteers/delete_volunteer.sql");
            sqlx::query(query).bind(id).execute(&mut **tx).await.context("error deleting user")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn batch_link_volunteers_to_nonprofits(
        &self,
        project_cycle_id: Uuid,
        linkage: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            project_cycle_id: Uuid,
            linkage: Vec<(Uuid, Uuid)>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let fragment =
                include_str!("queries/volunteers/link_volunteers_to_nonprofits.fragment.sql");

            // v.0 needs to be turned into an id by calling the get_volunteer_id function

            QueryBuilder::<Postgres>::new(fragment)
                .push_values(linkage, |mut b, (volunteer_id, nonprofit_id)| {
                    b.push_bind(project_cycle_id)
                        .push_bind(volunteer_id)
                        .push_bind(nonprofit_id)
                        .push_bind(true);
                })
                .build()
                .execute(&mut **tx)
                .await
                .context("error linking volunteers to nonprofits")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, linkage)
    }

    async fn batch_link_volunteers_to_mentors(
        &self,
        project_cycle_id: Uuid,
        linkage: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            project_cycle_id: Uuid,
            linkage: Vec<(Uuid, Uuid)>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let fragment =
                include_str!("queries/volunteers/link_volunteers_to_mentors.fragment.sql");

            // insert into volunteer_mentors(project_cycle_id, volunteer_id, mentor_id)

            QueryBuilder::<Postgres>::new(fragment)
                .push_values(linkage, |mut b, (volunteer_id, mentor_id)| {
                    b.push_bind(project_cycle_id).push_bind(volunteer_id).push_bind(mentor_id);
                })
                .build()
                .execute(&mut **tx)
                .await
                .map_err(|e| {
                    log::error!("error linking volunteers to mentors: {:?}", e);
                    e
                })?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, linkage)
    }

    async fn batch_insert_volunteers_exported_to_workspace(
        &self,
        data: Vec<InsertVolunteerExportedToWorkspace>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            data: Vec<InsertVolunteerExportedToWorkspace>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let fragment = include_str!(
                "queries/volunteers/batch_insert_volunteers_exported_to_workspace.fragment.sql"
            );

            QueryBuilder::<Postgres>::new(fragment)
                .push_values(data, |mut b, v| {
                    b.push_bind(v.volunteer_id)
                        .push_bind(v.job_id)
                        .push_bind(v.workspace_email)
                        .push_bind(v.org_unit);
                })
                .build()
                .execute(&mut **tx)
                .await?;

            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, data)
    }

    async fn batch_remove_volunteers_exported_to_workspace(
        &self,
        data: Vec<Uuid>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(data: Vec<Uuid>, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!(
                "queries/volunteers/batch_remove_volunteers_exported_to_workspace.sql"
            );

            sqlx::query(query).bind(data).execute(&mut **tx).await?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, data)
    }

    async fn fetch_exported_volunteer_details_by_project_cycle(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<ExportedVolunteerDetails>> {
        async fn exec(
            project_cycle_id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Vec<ExportedVolunteerDetails>> {
            let query = include_str!(
                "queries/volunteers/fetch_exported_volunteer_details_by_project_cycle.sql"
            );
            let volunteers = sqlx::query_as::<_, ExportedVolunteerDetails>(query)
                .bind(project_cycle_id)
                .fetch_all(&mut **tx)
                .await?;
            Ok(volunteers)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id)
    }
}
