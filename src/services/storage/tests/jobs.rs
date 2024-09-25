use anyhow::Result;
use sqlx::PgPool;
use uuid::uuid;

use crate::services::storage::{
    jobs::{CreateJob, EditJobBuilder, QueryJobs, UpdateJobStatus},
    types::{JobData, JobDetails, JobStatus, JobType},
    ExecOptsBuilder, PgBackend,
};

#[sqlx::test(fixtures("setup"))]
pub async fn test_create_job(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    let job_id = storage
        .create_job(
            None,
            CreateJob {
                label: "test".to_string(),
                description: Some("test".to_string()),
                data: JobDetails {
                    job_type: JobType::AirtableImportBase,
                    error: None,
                    data: JobData::AirtableImportBase {
                        base_id: "appS5z0uqz4l0IJvP".to_owned(),
                    },
                },
            },
            &mut exec_opts,
        )
        .await?;
    dbg!(job_id);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_job(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let job_id1 = uuid!("bc080e0d-8b14-46e0-9268-4bbb370035ec");
    let job_id2 = uuid!("413eed73-3c6f-456a-b9f0-ae72d136c742");

    let mut exec_opts = ExecOptsBuilder::default().build()?;

    let job1 = storage.fetch_job(job_id1, &mut exec_opts).await?;
    dbg!(&job1);

    let details = serde_json::from_value::<JobDetails>(job1.details)?;
    dbg!(details);

    let job2 = storage.fetch_job(job_id2, &mut exec_opts).await?;
    dbg!(&job2);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_update_job_status(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let job_id1 = uuid!("bc080e0d-8b14-46e0-9268-4bbb370035ec");

    let data = UpdateJobStatus {
        status: JobStatus::Error,
        error: Some("asdf".to_string()),
    };

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    // let job = storage.fetch_job(job_id1, &mut exec_opts).await?;
    // dbg!(&job);

    storage
        .update_job_status(job_id1, data, &mut exec_opts)
        .await?;
    let job = storage.fetch_job(job_id1, &mut exec_opts).await?;
    dbg!(&job);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_edit_job(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let job_id1 = uuid!("bc080e0d-8b14-46e0-9268-4bbb370035ec");
    let data = EditJobBuilder::default()
        .label("New Label".to_owned())
        .description("New description".to_owned())
        .build()?;

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    storage.edit_job(job_id1, data, &mut exec_opts).await?;

    let job = storage.fetch_job(job_id1, &mut exec_opts).await?;
    dbg!(&job);

    Ok(())
}
