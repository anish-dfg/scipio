//! This module contains the definition of the `QueryMentors` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::{Context, Result};
use async_trait::async_trait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use super::exec_with_tx;
use super::types::{MentorExperienceLevel, MentorYearsExperience, VolunteerHearAbout};
use crate::services::storage::entities::MentorDetails;
use crate::services::storage::{Acquire, ExecOpts, PgBackend};

/// Data needed to create a new mentor.
///
/// * `first_name`: The first name of the mentor
/// * `last_name`: The last name of the mentor
/// * `email`: The email of the mentor
/// * `phone`: The phone number of the mentor
/// * `offer_letter_signature`: Whether the mentor has signed the offer letter
/// * `company`: The company the mentor works for
/// * `job_title`: The job title of the mentor
#[derive(Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateMentor {
    // pub project_cycle_id: Uuid,
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub company: String,
    #[builder(setter(into))]
    pub job_title: String,
    #[builder(setter(into))]
    pub country: String,
    #[builder(setter(into), default = "None")]
    pub us_state: Option<String>,
    #[builder(setter(into))]
    pub years_experience: MentorYearsExperience,
    #[builder(setter(into))]
    pub experience_level: MentorExperienceLevel,
    #[builder(default = "false")]
    pub prior_mentor: bool,
    #[builder(default = "false")]
    pub prior_mentee: bool,
    #[builder(default = "false")]
    pub prior_student: bool,
    pub university: Vec<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
}

/// Data needed to edit a mentor.
///
/// All fields should be changed to optional. The SQL query should be updated to use `coalesce`.
///
/// * `first_name`: The new first name of the mentor
/// * `last_name`: The new last name of the mentor
/// * `email`: The new email of the mentor
/// * `phone`: The new phone numbe of the mentor
/// * `offer_letter_signature`: The new `offer_letter_signature` status of the mentor
// TODO: Update the EditMentor struct to make all fields optional
#[derive(Builder)]
pub struct EditMentor {
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    pub phone: String,
}

/// A trait for querying mentors.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryMentors<DB: Database> {
    /// Create a new mentor.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that the mentor is associated with.
    /// * `data`: Data required to create the mentor
    /// * `exec_opts`: Execution options for the query
    async fn create_mentor(
        &self,
        project_cycle_id: Uuid,
        data: CreateMentor,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Uuid> {
        unimplemented!()
    }

    /// Batch create mentors.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that all of the mentors are associated with.
    /// * `data`: The data needed to create the mentors
    /// * `exec_opts`: Execution options for the query
    async fn batch_create_mentors(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateMentor>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Vec<(String, Uuid)>> {
        unimplemented!()
    }

    /// Fetch all mentors.
    ///
    /// * `exec_opts`: Execution options for the query
    ///
    /// This function should be deprecated in favor of a version that takes filtering and pagination
    /// options.
    // TODO: Add a new function that fetches mentors with filtering and pagination options
    async fn fetch_mentors(&self, exec_opts: &mut ExecOpts<DB>) -> Result<Vec<MentorDetails>> {
        unimplemented!()
    }

    /// Fetch a mentor by ID.
    ///
    /// * `id`: The id of the mentor to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_mentor_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Option<MentorDetails>> {
        unimplemented!()
    }

    /// Edit a mentor.
    ///
    /// * `id`: The ID of the mentor to edit
    /// * `data`: Data required to edit the mentor
    /// * `exec_opts`: Execution options for the query
    async fn edit_mentor(
        &self,
        id: Uuid,
        data: EditMentor,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Delete a mentor by ID.
    ///
    /// * `id`: The id of the mentor to delete
    /// * `exec_opts`: Execution options for the query
    async fn delete_mentor(&self, id: Uuid, exec_opts: &mut ExecOpts<DB>) -> Result<()> {
        unimplemented!()
    }

    /// Batch link mentors to nonprofits.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that the mentors and nonprofits are associated with.
    /// * `data`: Data needed to link mentors to nonprofits
    /// * `exec_opts`: Execution options for the query
    async fn batch_link_mentors_to_nonprofits(
        &self,
        project_cycle_id: Uuid,
        data: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl QueryMentors<Postgres> for PgBackend {
    async fn create_mentor(
        &self,
        project_cycle_id: Uuid,
        data: CreateMentor,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Uuid> {
        async fn exec(
            project_cycle_id: Uuid,
            data: CreateMentor,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Uuid> {
            let query = include_str!("queries/mentors/create_mentor.sql");

            let id = sqlx::query_scalar::<_, Uuid>(query)
                .bind(project_cycle_id)
                .bind(data.first_name)
                .bind(data.last_name)
                .bind(data.email)
                .bind(data.phone)
                .bind(data.company)
                .bind(data.job_title)
                .bind(data.country)
                .bind(data.us_state)
                .bind(data.years_experience)
                .bind(data.experience_level)
                .bind(data.prior_mentor)
                .bind(data.prior_mentee)
                .bind(data.prior_student)
                .bind(data.university)
                .bind(data.hear_about)
                .fetch_one(&mut **tx)
                .await
                .context("error creating mentor")?;

            Ok(id)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn batch_create_mentors(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateMentor>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<(String, Uuid)>> {
        async fn exec(
            project_cycle_id: Uuid,
            data: Vec<CreateMentor>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Vec<(String, Uuid)>> {
            let fragment = include_str!("queries/mentors/create_mentors.fragment.sql");

            let data = QueryBuilder::<Postgres>::new(fragment)
                .push_values(data, |mut b, mentor| {
                    b.push_bind(project_cycle_id)
                        .push_bind(mentor.first_name)
                        .push_bind(mentor.last_name)
                        .push_bind(mentor.email)
                        .push_bind(mentor.phone)
                        .push_bind(mentor.company)
                        .push_bind(mentor.job_title)
                        .push_bind(mentor.country)
                        .push_bind(mentor.us_state)
                        .push_bind(mentor.years_experience)
                        .push_bind(mentor.experience_level)
                        .push_bind(mentor.prior_mentor)
                        .push_bind(mentor.prior_mentee)
                        .push_bind(mentor.prior_student)
                        .push_bind(mentor.university)
                        .push_bind(mentor.hear_about);
                })
                .push("returning email, id")
                .build_query_as::<(String, Uuid)>()
                .fetch_all(&mut **tx)
                .await?;

            Ok(data)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn fetch_mentors(
        &self,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<MentorDetails>> {
        async fn exec(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<MentorDetails>> {
            let query = include_str!("queries/mentors/fetch_mentors.sql");

            let mentors = sqlx::query_as::<_, MentorDetails>(query)
                .fetch_all(&mut **tx)
                .await
                .context("error fetching mentors")?;
            Ok(mentors)
        }

        exec_with_tx!(self, exec_opts, exec)
    }

    async fn fetch_mentor_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<MentorDetails>> {
        async fn exec(
            id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<MentorDetails>> {
            let query = include_str!("queries/mentors/fetch_mentor_by_id.sql");

            let mentor = sqlx::query_as::<_, MentorDetails>(query)
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .context("error fetching mentor by id")?;
            Ok(mentor)
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn edit_mentor(
        &self,
        id: Uuid,
        data: EditMentor,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            id: Uuid,
            data: EditMentor,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let query = include_str!("queries/mentors/edit_mentor.sql");

            sqlx::query(query)
                .bind(id)
                .bind(data.first_name)
                .bind(data.last_name)
                .bind(data.email)
                .bind(data.phone)
                .execute(&mut **tx)
                .await
                .context("error editing mentor")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id, data)
    }

    async fn delete_mentor(&self, id: Uuid, exec_opts: &mut ExecOpts<Postgres>) -> Result<()> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/mentors/delete_mentor.sql");
            sqlx::query(query)
                .bind(id)
                .execute(&mut **tx)
                .await
                .context("error deleting mentor")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn batch_link_mentors_to_nonprofits(
        &self,
        project_cycle_id: Uuid,
        data: Vec<(Uuid, Uuid)>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            project_cycle_id: Uuid,
            data: Vec<(Uuid, Uuid)>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let fragment = include_str!("queries/mentors/link_mentors_to_nonprofits.fragment.sql");

            QueryBuilder::<Postgres>::new(fragment)
                .push_values(data, |mut b, (mentor_id, nonprofit_id)| {
                    b.push_bind(project_cycle_id).push_bind(mentor_id).push_bind(nonprofit_id);
                })
                .build()
                .execute(&mut **tx)
                .await?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }
}
