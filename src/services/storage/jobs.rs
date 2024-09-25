//! This module contains the definition of the `QueryJobs` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use sqlx::{Database, Postgres, Transaction};
use uuid::Uuid;

use super::exec_with_tx;
use crate::services::storage::entities::Job;
use crate::services::storage::types::{JobDetails, JobStatus};
use crate::services::storage::{Acquire, ExecOpts, PgBackend};

/// Data needed to record a new asynchronous job.
///
/// * `label`: A friendly label for the new job
/// * `description`: A friendly description of the new job
/// * `data`: Details about the job
#[derive(Builder, Debug)]
pub struct CreateJob {
    #[builder(setter(into))]
    pub label: String,
    #[builder(setter(into))]
    pub description: Option<String>,
    pub data: JobDetails,
}

/// Data needed to update the status of a job.
///
/// * `status`: The new status
/// * `error`: Information about the error, if the new status is `Error`.
#[derive(Builder, Debug)]
pub struct UpdateJobStatus {
    pub status: JobStatus,
    pub error: Option<String>,
}

/// Data needed to edit a job.
///
/// * `label`: The new label for the job
/// * `description`: The new description for the job
#[derive(Builder, Debug)]
pub struct EditJob {
    #[builder(setter(into), default)]
    pub label: Option<String>,
    #[builder(setter(into), default)]
    pub description: Option<String>,
}

/// A trait for querying jobs.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryJobs<DB: Database> {
    /// Create a new job.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that the job is associated with, if it is
    ///   associated with one.
    /// * `data`: Data required to create the job
    /// * `exec_opts`: Execution options for the query
    async fn create_job(
        &self,
        project_cycle_id: Option<Uuid>,
        data: CreateJob,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Uuid> {
        unimplemented!()
    }

    /// Fetch all jobs.
    ///
    /// * `exec_opts`: Execution options for the query
    async fn fetch_jobs(&self, exec_opts: &mut ExecOpts<DB>) -> Result<Vec<Job>> {
        unimplemented!()
    }

    /// Fetch a job by ID.
    ///
    /// * `id`: The id of the job to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_job(&self, id: Uuid, exec_opts: &mut ExecOpts<DB>) -> Result<Job> {
        unimplemented!()
    }

    /// Update the status of a job.
    ///
    /// * `id`: The ID of the job to update
    /// * `data`: Data required to update the job status
    /// * `exec_opts`: Execution options for the query
    async fn update_job_status(
        &self,
        id: Uuid,
        data: UpdateJobStatus,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Set the project cycle that a job is associated with.
    ///
    /// This may be useful if a job is started to import data for a project cycle. The job is
    /// started before the cycle is created.
    ///
    /// * `id`: The id of the job to update
    /// * `project_cycle_id`: The new project cycle ID
    /// * `exec_opts`: Execution options for the query
    async fn set_job_project_cycle(
        &self,
        id: Uuid,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Edit a job.
    ///
    /// * `id`: The id of the job to edit
    /// * `data`: Data required to edit the job
    /// * `exec_opts`: Execution options for the query
    async fn edit_job(&self, id: Uuid, data: EditJob, opts: &mut ExecOpts<DB>) -> Result<()> {
        unimplemented!()
    }

    /// Cancel a job.
    ///
    /// * `id`: The id of the job to cancel
    /// * `opts`: Execution options for the query
    async fn cancel_job(&self, id: Uuid, opts: &mut ExecOpts<DB>) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl QueryJobs<Postgres> for PgBackend {
    async fn create_job(
        &self,
        project_cycle_id: Option<Uuid>,
        data: CreateJob,
        exec_opts: &mut ExecOpts,
    ) -> Result<Uuid> {
        async fn exec(
            project_cycle_id: Option<Uuid>,
            data: CreateJob,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Uuid> {
            let query = include_str!("queries/jobs/create_job.sql");
            let id = sqlx::query_scalar::<_, Uuid>(query)
                .bind(project_cycle_id)
                .bind(data.label)
                .bind(data.description)
                .bind(serde_json::to_value(data.data)?)
                .fetch_one(&mut **tx)
                .await?;
            Ok(id)
        }
        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn fetch_jobs(&self, exec_opts: &mut ExecOpts) -> Result<Vec<Job>> {
        async fn exec(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<Job>> {
            let query = include_str!("queries/jobs/fetch_jobs.sql");
            let jobs = sqlx::query_as::<_, Job>(query).fetch_all(&mut **tx).await?;
            Ok(jobs)
        }
        exec_with_tx!(self, exec_opts, exec)
    }

    async fn fetch_job(&self, id: Uuid, exec_opts: &mut ExecOpts) -> Result<Job> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<Job> {
            let query = include_str!("queries/jobs/fetch_job.sql");
            let job = sqlx::query_as::<_, Job>(query).bind(id).fetch_one(&mut **tx).await?;
            Ok(job)
        }
        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn update_job_status(
        &self,
        id: Uuid,
        data: UpdateJobStatus,
        exec_opts: &mut ExecOpts,
    ) -> Result<()> {
        async fn exec(
            id: Uuid,
            data: UpdateJobStatus,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let query = include_str!("queries/jobs/update_job_status.sql");
            sqlx::query(query)
                .bind(id)
                .bind(data.status)
                .bind(data.error)
                .execute(&mut **tx)
                .await?;
            Ok(())
        }
        exec_with_tx!(self, exec_opts, exec, id, data)
    }

    async fn set_job_project_cycle(
        &self,
        id: Uuid,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts,
    ) -> Result<()> {
        async fn exec(
            id: Uuid,
            project_cycle_id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let query = include_str!("queries/jobs/set_job_project_cycle.sql");
            sqlx::query(query).bind(id).bind(project_cycle_id).execute(&mut **tx).await?;
            Ok(())
        }
        exec_with_tx!(self, exec_opts, exec, id, project_cycle_id)
    }

    async fn edit_job(&self, id: Uuid, data: EditJob, opts: &mut ExecOpts) -> Result<()> {
        async fn exec(id: Uuid, data: EditJob, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/jobs/edit_job.sql");
            sqlx::query(query)
                .bind(id)
                .bind(data.label)
                .bind(data.description)
                .execute(&mut **tx)
                .await?;
            Ok(())
        }
        exec_with_tx!(self, opts, exec, id, data)
    }

    async fn cancel_job(&self, id: Uuid, exec_opts: &mut ExecOpts) -> Result<()> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/jobs/cancel_job.sql");
            sqlx::query(query).bind(id).execute(&mut **tx).await?;
            Ok(())
        }
        exec_with_tx!(self, exec_opts, exec, id)
    }
}
