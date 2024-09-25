use anyhow::Result;
use rstest::rstest;

use super::dfg_airtable_client;
use crate::services::airtable::base_data::records::{AirtableRecordsClient, ListRecordsQuery};
use crate::services::airtable::DfgAirtableClient;

#[rstest]
#[tokio::test]
pub async fn test_list_records(dfg_airtable_client: DfgAirtableClient) -> Result<()> {
    let query = ListRecordsQuery {
        page_size: Some(100),
        // fields: Some(
        //     [
        //         "FirstName",
        //         "LastName",
        //         "Email",
        //         "Phone",
        //         "Gender",
        //         "Ethnicity",
        //         "AgeRange",
        //         "University",
        //         "LGBT",
        //         "Country",
        //         "State",
        //         "FLI",
        //         "StudentStage",
        //         "Majors",
        //         "Minors",
        //         "HearAbout",
        //     ]
        //     .iter()
        //     .map(|s| s.to_string())
        //     .collect(),
        // ),
        // fields: Some(vec!["Name".to_owned()]),
        max_records: Some(100),

        view: Some("viwUn9qugk3tqMuKn".to_owned()),
        // view: Some("viwl9eu4orYdg0bnt".to_owned()),
        ..Default::default()
    };

    let _records = dfg_airtable_client
        // .list_records("appS5z0uqz4l0IJvP", "tblJfJ4klx6tXPLCQ", Some(query))
        .list_records("appS5z0uqz4l0IJvP", "tblLg3IAfj9lDjmeN", Some(query))
        .await?;
    dbg!(&_records);

    Ok(())
}
