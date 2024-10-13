use anyhow::Result;

use super::responses::{ListBasesResponse, SchemaResponse};
use crate::Airtable;

impl Airtable {
    pub async fn list_bases(&self, offset: Option<String>) -> Result<ListBasesResponse> {
        let mut url = "https://api.airtable.com/v0/meta/bases".to_owned();
        if let Some(offset) = offset {
            url.push_str(&format!("?offset={}", offset));
        }
        let data = self.http.get(&url).send().await?.json::<ListBasesResponse>().await?;
        Ok(data)
    }

    pub async fn get_base_schema(
        &self,
        base_id: &str,
        include: Vec<String>,
    ) -> Result<SchemaResponse> {
        let query =
            include.iter().map(|v| format!("include={}", v)).collect::<Vec<String>>().join("&");

        let url = format!("https://api.airtable.com/v0/meta/bases/{base_id}/tables?{query}");

        let data = self.http.get(url).send().await?.json::<SchemaResponse>().await?;

        Ok(data)
    }
}
