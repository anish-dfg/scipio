use anyhow::Result;
use sqlx::PgPool;
use uuid::uuid;

use crate::services::storage::{
    cycles::{CreateCycleBuilder, EditCycleBuilder, QueryCycles},
    ExecOptsBuilder, PgBackend,
};

#[sqlx::test(fixtures("setup"))]
pub async fn test_create_cycle(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let data = CreateCycleBuilder::default()
        .name("TestCycle")
        .description("Description of test cycle")
        .build()?;

    let id = storage
        .create_cycle(data, &mut ExecOptsBuilder::default().build()?)
        .await?;
    dbg!(id);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_cycles(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let cycles = storage
        .fetch_cycles(&mut ExecOptsBuilder::default().build()?)
        .await?;
    dbg!(cycles);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_cycle_by_id(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");
    let cycle = storage
        .fetch_cycle_by_id(cycle_id, &mut ExecOptsBuilder::default().build()?)
        .await?;
    dbg!(cycle);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_edit_cycle(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");
    let data = EditCycleBuilder::default()
        .name("Changed")
        .description("changed")
        .archived(true)
        .build()?;

    storage
        .edit_cycle(cycle_id, data, &mut ExecOptsBuilder::default().build()?)
        .await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_delete_cycle(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");

    storage
        .delete_cycle(cycle_id, &mut ExecOptsBuilder::default().build()?)
        .await?;
    Ok(())
}
