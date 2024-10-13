mod fixtures;

use anyhow::Result;
use fixtures::airtable;
use rstest::rstest;
use scipio_airtable::Airtable;

use crate::services::airtable::AirtableClient;

#[rstest]
#[tokio::test]
pub async fn test_list_mentors(airtable: Airtable) -> Result<()> {
    let mentors = airtable.list_mentors("appS5z0uqz4l0IJvP").await?;

    dbg!(&mentors);

    Ok(())
}
