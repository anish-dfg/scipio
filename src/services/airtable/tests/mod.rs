use std::env;

use anyhow::Result;
use rstest::{fixture, rstest};

use crate::services::airtable::base_data::bases::AirtableBasesClient;
use crate::services::airtable::DfgAirtableClient;

mod bases;
mod records;

#[fixture]
pub fn dfg_airtable_client() -> DfgAirtableClient {
    dotenvy::dotenv().expect("error loading environment variables");
    let api_token = env::var("AIRTABLE_API_TOKEN").expect("missing AIRTABLE_API_TOKEN variable");
    DfgAirtableClient::default_with_token(&api_token).expect("error constructing client")
}

#[rstest]
#[tokio::test]
pub async fn test_client_backoff_and_retry(dfg_airtable_client: DfgAirtableClient) -> Result<()> {
    // let _bases = dfg_airtable_client.list_bases(None).await.expect("error listing bases");
    // dbg!(_bases);
    for _ in 0..61 {
        let _bases = dfg_airtable_client.list_bases(None).await.expect("error listing bases");
        dbg!(_bases);
    }
    Ok(())
}
