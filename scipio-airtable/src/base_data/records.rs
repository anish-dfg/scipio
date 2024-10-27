use std::fmt::Display;

use anyhow::Result;
use derive_builder::Builder;
use derive_more::derive::Display;
use scipio_macros::ToQueryString;
use serde::{Deserialize, Serialize};

use super::responses::{GetRecordResponse, ListRecordsResponse};
use crate::Airtable;

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

#[derive(Debug, Serialize, Clone, Display)]
#[serde(rename_all = "snake_case")]
pub enum CellFormat {
    #[display("json")]
    Json,
    #[display("string")]
    String,
}

#[derive(Debug, Serialize, Clone, Builder, ToQueryString)]
#[serde(rename_all = "camelCase")]
pub struct GetRecordQuery {
    #[builder(setter(into), default)]
    pub time_zone: Option<String>,
    #[builder(default, setter(into))]
    pub user_locale: Option<String>,
    #[builder(setter(into), default)]
    pub cell_format: Option<CellFormat>,
    #[builder(default, setter(into))]
    pub return_fields_by_field_id: Option<bool>,
}

impl Airtable {
    pub async fn list_records<T>(
        &self,
        base_id: &str,
        table_id: &str,
        query: Option<&ListRecordsQuery>,
    ) -> Result<ListRecordsResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!(
            "https://api.airtable.com/v0/{base_id}/{table_id}/{query}",
            query = query.map(|q| q.to_query_string()).unwrap_or_default()
        );

        let data = self.http.get(&url).send().await?.json::<ListRecordsResponse<T>>().await?;

        Ok(data)
    }

    pub async fn get_record<T>(
        &self,
        base_id: &str,
        table_id: &str,
        record_id: &str,
        query: Option<&GetRecordQuery>,
    ) -> Result<GetRecordResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!(
            "https://api.airtable.com/v0/{base_id}/{table_id}/{record_id}/{query}",
            query = query.map(|q| q.to_query_string()).unwrap_or_default()
        );

        let data = self.http.get(&url).send().await?.json::<GetRecordResponse<T>>().await?;

        Ok(data)
    }

    pub async fn update_record<T>(
        &self,
        base_id: &str,
        table_id: &str,
        record_id: &str,
        data: T,
    ) -> Result<()>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(())
    }
}
