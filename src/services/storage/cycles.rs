//! This module contains the definition of the `QueryCycles` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use sqlx::{Database, Postgres, Transaction};
use uuid::Uuid;

use super::exec_with_tx;
use crate::services::storage::entities::ProjectCycle;
use crate::services::storage::{Acquire, ExecOpts, PgBackend};

#[derive(Builder)]
/// Data needed to create a new cycle.
///
/// * `name`: The name of the cycle
/// * `description`: A description of the cycle
pub struct CreateCycle {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub description: String,
}

/// Data needed to edit a cycle.
///
/// Every field should be changed to optional. The SQL query should be updated to use `coalesce`
///
/// * `name`: The new name of the cycle
/// * `description`: The new description of the cycle
/// * `archived`: The new archived state of the cycle
// TODO: Update the EditCycle struct to make all fields optional
#[derive(Builder)]
pub struct EditCycle {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub description: String,
    pub archived: bool,
}

/// A trait for querying cycles.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryCycles<DB: Database> {
    /// Creates a new cycle.
    ///
    /// * `data`: The data needed to create the cycle
    /// * `exec_opts`: Execution options for the query
    async fn create_cycle(&self, data: CreateCycle, exec_opts: &mut ExecOpts<DB>) -> Result<Uuid>;

    /// Fetches all cycles.
    ///
    /// * `exec_opts`: Execution options for the query
    ///
    /// This function should return all cycles, including archived ones. This function should also
    /// be deprecated in favor of a version that takes filtering and pagination options.
    // TODO: Add a new function that fetches cycles with filtering and pagination options
    async fn fetch_cycles(&self, exec_opts: &mut ExecOpts<DB>) -> Result<Vec<ProjectCycle>>;

    /// Fetches a cycle by ID.
    ///
    /// * `id`: The ID of the cycle to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_cycle_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<DB>,
    ) -> Result<Option<ProjectCycle>>;
    async fn edit_cycle(&self, _: Uuid, _: EditCycle, _: &mut ExecOpts<DB>) -> Result<()>;

    /// Deletes a cycle by ID.
    ///
    /// * `id`: The ID of the cycle to delete
    /// * `exec_opts`: Execution options for the query
    async fn delete_cycle(&self, id: Uuid, exec_opts: &mut ExecOpts<DB>) -> Result<()>;
}

#[async_trait]
impl QueryCycles<Postgres> for PgBackend {
    async fn create_cycle(
        &self,
        data: CreateCycle,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Uuid> {
        async fn exec(data: CreateCycle, tx: &mut Transaction<'_, Postgres>) -> Result<Uuid> {
            let query = include_str!("queries/cycles/create_cycle.sql");
            let id = sqlx::query_scalar::<_, Uuid>(query)
                .bind(data.name)
                .bind(data.description)
                .fetch_one(&mut **tx)
                .await?;
            Ok(id)
        }

        exec_with_tx!(self, exec_opts, exec, data)
    }

    async fn fetch_cycles(&self, exec_opts: &mut ExecOpts<Postgres>) -> Result<Vec<ProjectCycle>> {
        async fn exec(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<ProjectCycle>> {
            let query = include_str!("queries/cycles/fetch_cycles.sql");
            let cycles = sqlx::query_as::<_, ProjectCycle>(query).fetch_all(&mut **tx).await?;
            Ok(cycles)
        }

        exec_with_tx!(self, exec_opts, exec)
    }

    async fn fetch_cycle_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<ProjectCycle>> {
        async fn exec(
            id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<ProjectCycle>> {
            let query = "";
            let cycle =
                sqlx::query_as::<_, ProjectCycle>(query).bind(id).fetch_optional(&mut **tx).await?;
            Ok(cycle)
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn edit_cycle(
        &self,
        id: Uuid,
        data: EditCycle,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(id: Uuid, data: EditCycle, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = "";
            sqlx::query(query)
                .bind(id)
                .bind(data.name)
                .bind(data.description)
                .bind(data.archived)
                .execute(&mut **tx)
                .await?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id, data)
    }

    async fn delete_cycle(&self, id: Uuid, exec_opts: &mut ExecOpts<Postgres>) -> Result<()> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/cycles/delete_cycle.sql");
            sqlx::query(query).bind(id).execute(&mut **tx).await?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }
}
