use std::fmt::Display;

use anyhow::Result;
use derive_builder::Builder;
use scipio_macros::ToQueryString;
use serde::{Deserialize, Serialize};

use super::responses::ListRecordsResponse;
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

impl Airtable {
    pub async fn list_records<T>(
        &self,
        base_id: &str,
        table_id: &str,
        query: ListRecordsQuery,
    ) -> Result<ListRecordsResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!(
            "https://api.airtable.com/v0/{base_id}/{table_id}/{query}",
            base_id = base_id,
            table_id = table_id,
            query = query.to_query_string()
        );

        let data = self.http.get(&url).send().await?.json::<ListRecordsResponse<T>>().await?;

        Ok(data)
    }
}
