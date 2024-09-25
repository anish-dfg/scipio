//! This module defines the `AirtableRecordsClient` trait and implements it for `DfgAirtableClient`.

use std::fmt::Display;

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use pantheon_macros::ToQueryString;
use serde::Serialize;
use serde_json::Value;

use crate::services::airtable::base_data::records::responses::ListRecordsResponse;
use crate::services::airtable::DfgAirtableClient;

pub mod responses;

/// A struct representing a sort query parameter.
///
/// * `field`: The field to sort
/// * `direction`: The direction to sort in
#[derive(Debug, Serialize, Clone)]
pub struct Sort {
    pub field: String,
    pub direction: String,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.field, self.direction)
    }
}

/// Query parameters for listing records from a table.
///
/// Information about these parameters can be found
/// [here](https://airtable.com/developers/web/api/list-records)
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Serialize, Builder, ToQueryString, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListRecordsQuery {
    #[builder(setter(into), default = "Some(\"America/Los_Angeles\".to_owned())")]
    pub time_zone: Option<String>,
    #[builder(default, setter(into))]
    pub user_locale: Option<String>,
    #[builder(default = "Some(100)", setter(into))]
    pub page_size: Option<u8>,
    #[builder(default = "Some(100)", setter(into))]
    pub max_records: Option<u32>,
    #[builder(default, setter(into))]
    pub offset: Option<String>,
    #[builder(default, setter(into))]
    pub view: Option<String>,
    #[builder(default, setter(into))]
    pub sort: Option<Vec<Sort>>,
    #[builder(default, setter(into))]
    pub filter_by_formula: Option<String>,
    #[builder(default, setter(into))]
    pub cell_format: Option<String>,
    #[builder(default, setter(into))]
    pub fields: Option<Vec<String>>,
    #[builder(default, setter(into))]
    pub return_fields_by_field_id: Option<bool>,
    #[builder(default, setter(into))]
    pub record_metadata: Option<String>,
}

/// A trait to call the Airtable records API.
///
/// This trait is required to auto-impl `AirtableClient`
#[async_trait]
pub trait AirtableRecordsClient {
    /// List records from a table.
    ///
    /// * `base_id`: The base ID that the table is in
    /// * `table_id_or_name`: The ID or name of the table to list records from
    /// * `query`: Query parameters for the request
    async fn list_records(
        &self,
        base_id: &str,
        table_id_or_name: &str,
        query: Option<ListRecordsQuery>,
    ) -> Result<ListRecordsResponse<Value>>;
}

#[async_trait]
impl AirtableRecordsClient for DfgAirtableClient {
    async fn list_records(
        &self,
        base_id: &str,
        table_id_or_name: &str,
        query: Option<ListRecordsQuery>,
    ) -> Result<ListRecordsResponse<Value>> {
        let mut url = format!("https://api.airtable.com/v0/{base_id}/{table_id_or_name}");
        if let Some(ref qs) = query {
            url.push_str(&qs.to_query_string());
        }

        let data = self.http.get(url).send().await?.json::<ListRecordsResponse<Value>>().await?;

        Ok(data)
    }
}
