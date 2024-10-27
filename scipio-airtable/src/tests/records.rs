use std::env;

use anyhow::Result;
use rstest::rstest;

use super::fixtures::{context, AsyncTestContext};
use super::Customer;
use crate::base_data::records::{GetRecordQueryBuilder, ListRecordsQueryBuilder};

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
pub async fn test_list_records(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    cleanup.push(Box::new(|| {
        Box::pin(async move {
            println!("Cleaning up test_list_records");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");
    let view = env::var("TEST_AIRTABLE_API_VIEW").expect("missing TEST_AIRTABLE_VIEW variable");

    let query = ListRecordsQueryBuilder::default()
        .fields(Customer::field_names().iter().map(ToString::to_string).collect::<Vec<_>>())
        .view(view)
        .build()?;

    let res = context.airtable.list_records::<Customer>(&base, &table, Some(&query)).await?;

    dbg!(&res.records);

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
pub async fn test_get_record(context: AsyncTestContext) -> Result<()> {
    use anyhow::bail;

    let mut cleanup = context.cleanup.lock().await;
    cleanup.push(Box::new(|| {
        Box::pin(async move {
            log::info!("Cleaning up test_get_record");
            bail!("test_get_record failed");
            Ok(())
        })
    }));

    let base = env::var("TEST_AIRTABLE_API_BASE").expect("missing TEST_AIRTABLE_BASE variable");
    let table = env::var("TEST_AIRTABLE_API_TABLE").expect("missing TEST_AIRTABLE_TABLE variable");
    let record_id =
        env::var("TEST_AIRTABLE_API_RECORD_ID").expect("missing TEST_AIRTABLE_RECORD_ID variable");

    let query = GetRecordQueryBuilder::default().build()?;

    let res =
        context.airtable.get_record::<Customer>(&base, &table, &record_id, Some(&query)).await?;

    dbg!(&res);

    Ok(())
}

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
pub async fn test_update_record(context: AsyncTestContext) -> Result<()> {
    let mut cleanup = context.cleanup.lock().await;
    let airtable = context.airtable.clone();
    cleanup.push(Box::new(move || {
        let airtable = airtable.clone();
        Box::pin(async move {
            log::info!("Cleaning up test_get_record");
            Ok(())
        })
    }));

    Ok(())
}
