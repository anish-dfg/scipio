//! This module contains the definition of the `QueryStats` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Database, Postgres, Transaction};
use uuid::Uuid;

use super::{exec_with_tx, Acquire};
use crate::services::storage::entities::BasicStats;
use crate::services::storage::{ExecOpts, PgBackend};

/// A trait for querying statistics about the data in Pantheon.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryStats<DB: Database> {
    /// Get basic statistics about a project cycle in Pantheon.
    ///
    /// * `project_cycle_id`: The ID of the project cycle to get statistics for
    /// * `exec_opts`: Execution options for the query
    async fn get_basic_stats(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<BasicStats> {
        unimplemented!()
    }
}

#[async_trait]
impl QueryStats<Postgres> for PgBackend {
    async fn get_basic_stats(
        &self,
        project_cycle_id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<BasicStats> {
        async fn exec(
            project_cycle_id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<BasicStats> {
            let query = include_str!("queries/stats/fetch_basic_stats.sql");
            let stats = sqlx::query_as::<_, BasicStats>(query)
                .bind(project_cycle_id)
                .fetch_one(&mut **tx)
                .await?;
            Ok(stats)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id)
    }
}
