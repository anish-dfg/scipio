//! This module contains traits for interacting with the database, as well as one concrete
//! implementation (Postgres).

pub mod cycles;
pub mod entities;
pub mod jobs;
pub mod mentors;
pub mod nonprofits;
pub mod stats;
pub mod types;
pub mod volunteers;

#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use async_trait::async_trait;
use derive_builder::Builder;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Database, PgPool, Postgres, Transaction};

use crate::services::storage::cycles::QueryCycles;
use crate::services::storage::jobs::QueryJobs;
use crate::services::storage::mentors::QueryMentors;
use crate::services::storage::nonprofits::QueryNonprofits;
use crate::services::storage::stats::QueryStats;
use crate::services::storage::volunteers::QueryVolunteers;

/// Defines the storage layer for the application.
///
/// This trait is an auto trait, which means that
/// it's implemented for any type that implements the required traits. It is parameterized by a
/// `sqlx::Database` type, which is used to specify the database backend. The default is
/// `Postgres`, and it's the only one with a concrete implementation.
pub trait StorageLayer<DB: Database = Postgres>:
    QueryVolunteers<DB>
    + QueryMentors<DB>
    + QueryNonprofits<DB>
    + QueryCycles<DB>
    + QueryJobs<DB>
    + QueryStats<DB>
    + Acquire<DB>
    + Send
    + Sync
{
}

/// Defines a backend for the Postgres database. It contains a connection pool.
///
/// * `pool`: A Postgres connection pool
pub struct PgBackend {
    pub pool: PgPool,
}

/// `Migrator` is a trait for running migrations on a database.
#[async_trait]
pub trait Migrator {
    async fn migrate(&self) -> Result<()>;
}

/// `Acquire` is a trait for acquiring a transaction from a database.
#[async_trait]
pub trait Acquire<DB: Database> {
    async fn acquire<'a>(&self) -> Result<Transaction<'a, DB>>;
}

impl PgBackend {
    /// Construct a new `PgBackend` from a URL.
    ///
    /// * `url`: A string slice that holds a valid Postgres URL, e.g.
    ///   `postgresql://username:password@host:port/database?key1=value1&key2=value2`
    pub async fn new(url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(url)
            .await
            .context("create pgpool")?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Acquire<Postgres> for PgBackend {
    async fn acquire<'a>(&self) -> Result<Transaction<'a, Postgres>> {
        Ok(self.pool.begin().await.context("acquire transaction")?)
    }
}

#[async_trait]
impl Migrator for PgBackend {
    async fn migrate(&self) -> Result<()> {
        sqlx::migrate!().run(&self.pool).await?;
        Ok(())
    }
}

/// `ExecOpts` is a struct that holds options for executing a query.
///
/// * `tx`: An optional mutable reference to a transaction
///
/// `ExecOpts` is a foundational struct in the storage layer. It is used to pass options, including
/// transaction data, to every query method. It is parameterized by a `sqlx::Database` database type
/// `DB` which defaults to `Postgres`.
#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct ExecOpts<'a, DB: Database = Postgres> {
    #[builder(setter(into), default = "None")]
    pub tx: Option<&'a mut Transaction<'static, DB>>,
}

/// `exec_with_tx` is a macro that executes a query within a transaction.
///
/// If no transaction is provided in the options passed to the method, this macro acquires one from
/// the pool, executes the query, and commits the transaction. If a transaction is provided, it will
/// execute the query against the transaction, but will not commit it. It has two branches: one for
/// queries that require additional arguments, and one for queries that do not. It works this way
/// because each query method defines an inner function conventionally called `exec`, which takes a
/// transaction and 0 or more additional arguments. The macro calls this inner function with the
/// additional arguments and the transaction, or just the transaction if there are no additional
/// arguments.
macro_rules! exec_with_tx {
    // Branch with additional arguments
    ($self:expr, $exec_opts:expr, $exec_fn:ident, $( $arg:expr ),* ) => {
        match $exec_opts.tx {
            Some(ref mut tx) => $exec_fn($( $arg ),*, tx).await,
            _ => {
                let mut tx = $self.acquire().await?;
                let res = $exec_fn($( $arg ),*, &mut tx).await;
                tx.commit().await?;
                res
            }
        }
    };
    // Branch without additional arguments
    ($self:expr, $exec_opts:expr, $exec_fn:ident) => {
        match $exec_opts.tx {
            Some(ref mut tx) => $exec_fn(tx).await,
            _ => {
                let mut tx = $self.acquire().await?;
                let res = $exec_fn(&mut tx).await;
                tx.commit().await?;
                res
            }
        }
    };
}

pub(in crate::services::storage) use exec_with_tx;

// Auto-impl StorageLayer for any type that implements the required traits
impl<T, DB: Database> StorageLayer<DB> for T where
    T: QueryVolunteers<DB>
        + QueryMentors<DB>
        + QueryNonprofits<DB>
        + QueryCycles<DB>
        + QueryJobs<DB>
        + QueryStats<DB>
        + Acquire<DB>
        + Migrator
        + Send
        + Sync
{
}
