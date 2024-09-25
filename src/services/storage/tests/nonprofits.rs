use anyhow::Result;
use sqlx::PgPool;
use uuid::{uuid, Uuid};

use crate::services::storage::nonprofits::{
    CreateNonprofitBuilder, EditNonprofitBuilder, QueryNonprofits,
};
use crate::services::storage::types::{ClientSize, ImpactCause};
use crate::services::storage::{Acquire, ExecOptsBuilder, PgBackend};

#[sqlx::test(fixtures("setup"))]
pub async fn test_create_nonprofit(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("76ed64a0-d88f-4148-9b02-331ea888d5d1");
    let data = CreateNonprofitBuilder::default()
        .representative_first_name("Venus")
        .representative_last_name("Williams")
        .representative_job_title("Tennis Champion")
        .email("venus.williams@gmail.com")
        .email_cc("serena.williams@gmail.com".to_string())
        .phone("989-989-9881")
        .org_name("Williams Tennis Nonprofit")
        .project_name("Williams Tennis Website Refresh")
        .org_website("venus.example.com".to_string())
        .address("Somewhere in LA")
        .size(ClientSize::S101_500)
        .impact_causes(vec![ImpactCause::CareerAndProfessionalDevelopment])
        .build()?;

    let mut tx = storage.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;

    let id = storage.create_nonprofit(project_cycle_id, data, &mut exec_opts).await?;

    tx.commit().await?;

    dbg!(&id);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_batch_create_nonprofits(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("76ed64a0-d88f-4148-9b02-331ea888d5d1");

    let data = vec![
        CreateNonprofitBuilder::default()
            .representative_first_name("Iga")
            .representative_last_name("Swiatek")
            .representative_job_title("Tennis Champion")
            .email("iga.swiatek@gmail.com")
            .phone("989-989-9882")
            .org_name("Swiatek Tennis Organization")
            .project_name("Swiatek Org Website Refresh")
            .org_website("iga.example.com".to_string())
            .address("Somewhere in Warsaw")
            .size(ClientSize::S21_50)
            .impact_causes(vec![ImpactCause::CareerAndProfessionalDevelopment])
            .build()?,
        CreateNonprofitBuilder::default()
            .representative_first_name("Aryna")
            .representative_last_name("Sabalenka")
            .representative_job_title("Tennis Champion")
            .email("aryna.sabalenka@gmail.com")
            .phone("989-989-9883")
            .org_name("Sabelenka Nonprofit")
            .project_name("Sabalenka App Refresh")
            .org_website("aryna.example.com".to_string())
            .address("Somewhere in Miami")
            .size(ClientSize::S6_20)
            .impact_causes(vec![ImpactCause::CareerAndProfessionalDevelopment])
            .build()?,
    ];

    let mut tx = storage.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;
    let nonprofits =
        storage.batch_create_nonprofits(project_cycle_id, data, &mut exec_opts).await?;

    dbg!(&nonprofits);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_nonprofits(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };

    let nonprofits = storage.fetch_nonprofits(&mut ExecOptsBuilder::default().build()?).await?;
    dbg!(&nonprofits);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_nonprofit_by_id(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let nonprofit_id = uuid!("bb9b7fa5-7283-4b73-82e1-c7244e47421d");
    let fake_nonprofit_id = Uuid::default();

    let nonprofit = storage
        .fetch_nonprofit_by_id(nonprofit_id, &mut ExecOptsBuilder::default().build()?)
        .await?;

    assert!(nonprofit.is_some());

    let nonexistent = storage
        .fetch_nonprofit_by_id(fake_nonprofit_id, &mut ExecOptsBuilder::default().build()?)
        .await?;
    assert!(nonexistent.is_none());

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_nonprofit_by_org_name(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let org_name = "AgassiOrg";
    let fake_org_name = "LaverOrg";

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    let nonprofit = storage.fetch_nonprofit_by_org_name(org_name, &mut exec_opts).await?;

    assert!(nonprofit.is_some());

    let nonexistent = storage.fetch_nonprofit_by_org_name(fake_org_name, &mut exec_opts).await?;

    assert!(nonexistent.is_none());

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_edit_nonprofit(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let nonprofit_id = uuid!("bb9b7fa5-7283-4b73-82e1-c7244e47421d");

    let data = EditNonprofitBuilder::default()
        .email("pete.sampras@gmail.com")
        .email_cc("jimmy.connors@gmail.com".to_string())
        .phone("857-231-5830")
        .build()?;

    let mut exec_opts = ExecOptsBuilder::default().build()?;

    storage.edit_nonprofit(nonprofit_id, data, &mut exec_opts).await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_delete_nonprofit(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let nonprofit_id = uuid!("bb9b7fa5-7283-4b73-82e1-c7244e47421d");
    let mut exec_opts = ExecOptsBuilder::default().build()?;

    storage.delete_nonprofit(nonprofit_id, &mut exec_opts).await?;
    Ok(())
}
