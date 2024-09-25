use anyhow::Result;
use rstest::rstest;

use crate::services::airtable::base_data::bases::AirtableBasesClient;
use crate::services::airtable::tests::dfg_airtable_client;
use crate::services::airtable::DfgAirtableClient;

#[rstest]
#[tokio::test]
pub async fn test_list_bases(dfg_airtable_client: DfgAirtableClient) -> Result<()> {
    let _bases = dfg_airtable_client.list_bases(None).await.expect("error listing bases");
    dbg!(_bases);
    Ok(())
}

#[rstest]
#[tokio::test]
pub async fn test_get_base_schema(dfg_airtable_client: DfgAirtableClient) -> Result<()> {
    let test_base_id = "appXkJtMAdIHc0urz";
    let _schema = dfg_airtable_client
        .get_base_schema(test_base_id, None)
        .await
        .expect("error getting base schema");

    dbg!(_schema);

    Ok(())
}
