//! This module contains the definition of the `QueryMentors` trait as well as the default
//! implementation of the trait for the `PgBackend` struct.

use anyhow::{Context, Result};
use async_trait::async_trait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use super::entities::NonprofitClientDetails;
use super::types::{ClientSize, ImpactCause};
use super::{exec_with_tx, PgBackend};
use crate::services::storage::{Acquire, ExecOpts};

/// Data needed to create a new nonprofit.
///
/// * `representative_first_name`: The first name of the nonprofit representative
/// * `representative_last_name`: The last name of the nonprofit representative
/// * `representative_job_title`: The job title of the nonprofit representative
/// * `email`: The nonprofit's email address
/// * `email_cc`: The email address to CC on emails when sending email to the nonprofit's email
/// * `phone`: The phone number of the nonprofit
/// * `org_name`: The name of the nonprofit organization
/// * `project_name`: The name of this nonprofit's project for this cycle
/// * `org_website`: The website of the nonprofit organization
/// * `country_hq`: The country where the nonprofit is headquartered
/// * `state_hq`: The state or province where the nonprofit is headquartered
/// * `address`: The address of the nonprofit
/// * `size`: The size of the nonprofit
#[derive(Builder, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CreateNonprofit {
    #[builder(setter(into))]
    pub representative_first_name: String,
    #[builder(setter(into))]
    pub representative_last_name: String,
    #[builder(setter(into))]
    pub representative_job_title: String,
    #[builder(setter(into))]
    pub email: String,
    #[serde(rename = "Cc")]
    #[builder(setter(into), default = "None")]
    pub email_cc: Option<String>,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub org_name: String,
    #[builder(setter(into))]
    pub project_name: String,
    #[builder(setter(into), default = "None")]
    pub org_website: Option<String>,
    #[serde(rename = "CountryHQ")]
    #[builder(setter(into), default = "None")]
    pub country_hq: Option<String>,
    #[serde(rename = "StateHQ")]
    #[builder(setter(into), default = "None")]
    pub us_state_hq: Option<String>,
    #[builder(setter(into))]
    pub address: String,
    pub size: ClientSize,
    pub impact_causes: Vec<ImpactCause>,
}

/// Data needed to edit a nonprofit.
///
/// * `email`: The new email for the nonprofit
/// * `email_cc`: The new email to CC on emails when sending email to the nonprofit's email
/// * `phone`: The new phone number for the nonprofit
/// * `org_website`: The new website for the nonprofit
///
/// Every field should be changed to optional. The SQL query should be updated to use `coalesce`
// TODO: Update the EditNonprofit struct to make all fields optional
#[derive(Builder)]
pub struct EditNonprofit {
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub email_cc: Option<String>,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into), default = "None")]
    pub org_website: Option<String>,
}

/// A trait for querying nonprofits.
///
/// If you implement a new storage backend, this trait is required for it to implement
/// `StorageLayer`. The default implementation is for `Postgres`.
#[async_trait]
#[allow(unused)]
pub trait QueryNonprofits<DB: Database> {
    /// Create a new nonprofit.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that the nonprofit is associated with
    /// * `data`: Data required to create the nonprofit
    /// * `exec_opts`: Execution options for the query
    async fn create_nonprofit(
        &self,
        project_cycle_id: Uuid,
        data: CreateNonprofit,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Uuid> {
        unimplemented!()
    }

    /// Batch create new nonprofits.
    ///
    /// * `project_cycle_id`: The ID of the project cycle that the nonprofits are associated with
    /// * `data`: The data required to create the nonprofits
    /// * `exec_opts`: Execution options for the query
    async fn batch_create_nonprofits(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateNonprofit>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<(String, Uuid)>> {
        unimplemented!()
    }

    /// Fetch all nonprofits.
    ///
    /// * `exec_opts`: Execution options for the query
    ///
    /// This function should be deprecated in favor of a version that takes filtering and pagination options.
    // TODO: Add a new function that fetches nonprofits with filtering and pagination options
    async fn fetch_nonprofits(
        &self,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<NonprofitClientDetails>> {
        unimplemented!()
    }

    /// Fetch a nonprofit by ID.
    ///
    /// * `id`: The ID of the nonprofit to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_nonprofit_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<NonprofitClientDetails>> {
        unimplemented!()
    }

    /// Fetch a nonprofit by organization name.
    ///
    /// * `org_name`: The name of the organization to fetch
    /// * `exec_opts`: Execution options for the query
    async fn fetch_nonprofit_by_org_name(
        &self,
        org_name: &str,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<NonprofitClientDetails>> {
        unimplemented!()
    }

    /// Edit a nonprofit.
    ///
    /// * `id`: The ID of the nonprofit to edit
    /// * `data`: Data required to edit the nonprofit
    /// * `exec_opts`: Execution options for the query
    async fn edit_nonprofit(
        &self,
        id: Uuid,
        data: EditNonprofit,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Delete a nonprofit.
    ///
    /// * `id`: The ID of the nonprofit to delete
    /// * `exec_opts`: Execution options for the query
    async fn delete_nonprofit(&self, id: Uuid, exec_opts: &mut ExecOpts<Postgres>) -> Result<()> {
        unimplemented!()
    }
}

#[async_trait]
impl QueryNonprofits<Postgres> for PgBackend {
    async fn create_nonprofit(
        &self,
        project_cycle_id: Uuid,
        data: CreateNonprofit,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Uuid> {
        async fn exec(
            project_cycle_id: Uuid,
            data: CreateNonprofit,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Uuid> {
            let query = include_str!("queries/nonprofits/create_nonprofit.sql");

            let id = sqlx::query_scalar::<_, Uuid>(query)
                .bind(project_cycle_id)
                .bind(data.representative_first_name)
                .bind(data.representative_last_name)
                .bind(data.representative_job_title)
                .bind(data.email)
                .bind(data.email_cc)
                .bind(data.phone)
                .bind(data.org_name)
                .bind(data.project_name)
                // .bind(data.agreement_and_invoice_sent)
                // .bind(data.services_agreement_signature)
                // .bind(data.availability_confirmed)
                // .bind(data.invoice_paid)
                .bind(data.org_website)
                .bind(data.country_hq)
                .bind(data.us_state_hq)
                .bind(data.address)
                .bind(data.size)
                .bind(data.impact_causes)
                .fetch_one(&mut **tx)
                .await
                .context("error creating nonprofit")?;
            Ok(id)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn batch_create_nonprofits(
        &self,
        project_cycle_id: Uuid,
        data: Vec<CreateNonprofit>,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<(String, Uuid)>> {
        async fn exec(
            project_cycle_id: Uuid,
            data: Vec<CreateNonprofit>,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Vec<(String, Uuid)>> {
            let fragment = include_str!("queries/nonprofits/create_nonprofits.fragment.sql");

            let data = QueryBuilder::<Postgres>::new(fragment)
                .push_values(data, |mut b, n| {
                    b.push_bind(project_cycle_id)
                        .push_bind(n.representative_first_name)
                        .push_bind(n.representative_last_name)
                        .push_bind(n.representative_job_title)
                        .push_bind(n.email)
                        .push_bind(n.email_cc)
                        .push_bind(n.phone)
                        .push_bind(n.org_name)
                        .push_bind(n.project_name)
                        // .push_bind(n.agreement_and_invoice_sent)
                        // .push_bind(n.services_agreement_signature)
                        // .push_bind(n.availability_confirmed)
                        // .push_bind(n.invoice_paid)
                        .push_bind(n.org_website)
                        .push_bind(n.country_hq)
                        .push_bind(n.us_state_hq)
                        .push_bind(n.address)
                        .push_bind(n.size)
                        .push_bind(n.impact_causes);
                })
                .push("returning org_name, id")
                .build_query_as::<(String, Uuid)>()
                .fetch_all(&mut **tx)
                .await?;
            // .map_err(|e| e.into())
            // .context("error creating nonprofits: {:#}")?;
            Ok(data)
        }

        exec_with_tx!(self, exec_opts, exec, project_cycle_id, data)
    }

    async fn fetch_nonprofits(
        &self,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Vec<NonprofitClientDetails>> {
        async fn exec(tx: &mut Transaction<'_, Postgres>) -> Result<Vec<NonprofitClientDetails>> {
            let query = include_str!("queries/nonprofits/fetch_nonprofits.sql");
            let nonprofits = sqlx::query_as::<_, NonprofitClientDetails>(query)
                .fetch_all(&mut **tx)
                .await
                .context("error fetching nonprofits")?;

            Ok(nonprofits)
        }

        exec_with_tx!(self, exec_opts, exec)
    }

    async fn fetch_nonprofit_by_id(
        &self,
        id: Uuid,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<NonprofitClientDetails>> {
        async fn exec(
            id: Uuid,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<NonprofitClientDetails>> {
            let query = include_str!("queries/nonprofits/fetch_nonprofit_by_id.sql");

            let nonprofit = sqlx::query_as::<_, NonprofitClientDetails>(query)
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .context("error fetching nonprofit by id")?;
            Ok(nonprofit)
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }

    async fn fetch_nonprofit_by_org_name(
        &self,
        org_name: &str,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<Option<NonprofitClientDetails>> {
        async fn exec<'b>(
            org_name: &'b str,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<Option<NonprofitClientDetails>> {
            let query = include_str!("queries/nonprofits/fetch_nonprofit_by_org_name.sql");

            let nonprofit = sqlx::query_as::<_, NonprofitClientDetails>(query)
                .bind(org_name)
                .fetch_optional(&mut **tx)
                .await
                .context("error fetching nonprofit by org name")?;
            Ok(nonprofit)
        }

        exec_with_tx!(self, exec_opts, exec, org_name)
    }

    async fn edit_nonprofit(
        &self,
        id: Uuid,
        data: EditNonprofit,
        exec_opts: &mut ExecOpts<Postgres>,
    ) -> Result<()> {
        async fn exec(
            id: Uuid,
            data: EditNonprofit,
            tx: &mut Transaction<'_, Postgres>,
        ) -> Result<()> {
            let query = include_str!("queries/nonprofits/edit_nonprofit.sql");
            sqlx::query(query)
                .bind(id)
                .bind(data.email)
                .bind(data.email_cc)
                .bind(data.phone)
                .bind(data.org_website)
                .execute(&mut **tx)
                .await
                .context("error editing nonprofit")?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id, data)
    }

    async fn delete_nonprofit(&self, id: Uuid, exec_opts: &mut ExecOpts<Postgres>) -> Result<()> {
        async fn exec(id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<()> {
            let query = include_str!("queries/nonprofits/delete_nonprofit.sql");
            sqlx::query(query).bind(id).execute(&mut **tx).await?;
            Ok(())
        }

        exec_with_tx!(self, exec_opts, exec, id)
    }
}
