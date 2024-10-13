use anyhow::Result;
use rstest::rstest;
use serde::Deserialize;

use super::fixtures::airtable;
use crate::base_data::records::ListRecordsQueryBuilder;
use crate::Airtable;

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
pub async fn test_list_records(airtable: Airtable) -> Result<()> {
    #[allow(unused)]
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct SubsetFields {
        first_name: String,
        last_name: String,
        email: String,
        phone: Option<String>,
    }

    #[allow(unused)]
    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "PascalCase")]
    pub struct MentorMenteeLinkage {
        #[serde(rename = "Email")]
        pub mentor_email: String,
        #[serde(rename = "Mentee Email (from Volunteers)", default)]
        pub mentee_email: Vec<String>,
    }

    let query = ListRecordsQueryBuilder::default()
        .fields(
            // ["FirstName", "LastName", "Email", "Phone"]
            ["Email", "Mentee Email (from Volunteers)"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        )
        .view("viwifNGQcjzu4d3wQ".to_owned())
        .build()?;

    let res = airtable
        .list_records::<MentorMenteeLinkage>("appS5z0uqz4l0IJvP", "tblJfJ4klx6tXPLCQ", query)
        .await?;

    dbg!(&res.records);

    Ok(())
}
