//! This module defines the `AirtableBasesClient` trait and implements it for `DfgAirtableClient`.

pub mod responses;

use anyhow::Result;
use async_trait::async_trait;

use self::responses::ListBasesResponse;
use crate::services::airtable::base_data::bases::responses::SchemaResponse;
use crate::services::airtable::DfgAirtableClient;
use crate::services::url::UrlBuilder;

/// A trait to call the Airtable bases API.
///
/// This trait is required to auto-impl `AirtableClient`
#[async_trait]
#[allow(unused)]
pub trait AirtableBasesClient {
    /// List all Airtable bases (allowed by the permissions on the API token).
    ///
    /// * `offset`: The offset to start listing bases from (essentially a pagination token)
    async fn list_bases(&self, offset: Option<String>) -> Result<ListBasesResponse>;

    /// Get the schema for a base.
    ///
    /// * `base_id`: The ID of the base to get the schema for
    /// * `include`: Additional fields to include in the views response object
    async fn get_base_schema(
        &self,
        base_id: &str,
        include: Option<Vec<String>>,
    ) -> Result<SchemaResponse>;
}

#[async_trait]
impl AirtableBasesClient for DfgAirtableClient {
    async fn list_bases(&self, offset: Option<String>) -> Result<ListBasesResponse> {
        let mut builder = UrlBuilder::parse("https://api.airtable.com/v0/meta/bases")?;

        if let Some(ref offset) = offset {
            builder.push_query(("offset".to_owned(), offset.to_owned()));
        };

        let url = builder.build()?.to_string();

        let data = self.http.get(url).send().await?.json::<ListBasesResponse>().await?;

        Ok(data)
    }

    async fn get_base_schema(
        &self,
        base_id: &str,
        include: Option<Vec<String>>,
    ) -> Result<SchemaResponse> {
        let query = match include {
            Some(ref include) => include
                .iter()
                .cloned()
                .map(|v| ("include".to_owned(), v))
                .collect::<Vec<(String, String)>>(),

            None => vec![],
        };

        let url = UrlBuilder::parse("https://api.airtable.com/v0/meta/bases")?
            .push_path(base_id)
            .push_path("tables")
            .query(query)
            .build()?
            .to_string();

        let data = self.http.get(url).send().await?.json::<SchemaResponse>().await?;

        Ok(data)
    }
}
