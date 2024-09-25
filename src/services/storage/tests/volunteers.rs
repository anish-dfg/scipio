use anyhow::Result;
use sqlx::PgPool;
use uuid::uuid;

use crate::services::storage::types::{
    AgeRange, Ethnicity, Fli, Gender, Lgbt, StudentStage, VolunteerHearAbout,
};
use crate::services::storage::volunteers::{
    CreateVolunteerBuilder, EditVolunteerBuilder, InsertVolunteerExportedToWorkspaceBuilder,
    QueryVolunteers,
};
use crate::services::storage::{Acquire, ExecOptsBuilder, PgBackend};

#[sqlx::test(fixtures("setup"))]
pub async fn test_create_volunteer(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");
    let data = CreateVolunteerBuilder::default()
        .first_name("Carlos")
        .last_name("Alcaraz")
        .email("carlos.alcaraz@gmail.com")
        .volunteer_gender(Gender::Man)
        .volunteer_ethnicity(vec![Ethnicity::LatinoOrHispanic])
        .volunteer_age_range(AgeRange::R18_24)
        .university(vec!["University of Murcia".to_owned()])
        .lgbt(Lgbt::No)
        .country("Spain".to_owned())
        .fli(vec![Fli::FirstGeneration])
        .student_stage(StudentStage::Freshman)
        .majors(vec!["Business Administration".to_owned()])
        .minors(vec!["Economics".to_owned()])
        .hear_about(vec![VolunteerHearAbout::University])
        .build()?;

    let mut tx = storage.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;

    let id = storage.create_volunteer(project_cycle_id, data, &mut exec_opts).await?;
    tx.commit().await?;

    dbg!(id);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_batch_create_volunteers(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");

    let data = vec![
        CreateVolunteerBuilder::default()
            .first_name("Jannik")
            .last_name("Sinner")
            .email("jannik.sinner@gmail.com")
            .volunteer_gender(Gender::Man)
            .volunteer_ethnicity(vec![Ethnicity::WhiteOrCaucasian])
            .volunteer_age_range(AgeRange::R18_24)
            .university(vec!["University of Turin".to_owned()])
            .lgbt(Lgbt::No)
            .country("Italy".to_owned())
            .fli(vec![Fli::FirstGeneration])
            .student_stage(StudentStage::Freshman)
            .majors(vec!["Business Administration".to_owned()])
            .minors(vec!["Economics".to_owned()])
            .hear_about(vec![VolunteerHearAbout::University])
            .build()?,
        CreateVolunteerBuilder::default()
            .first_name("Daniil")
            .last_name("Medvedev")
            .email("daniil.medvedev@gmail.com")
            .volunteer_gender(Gender::Man)
            .volunteer_ethnicity(vec![Ethnicity::WhiteOrCaucasian])
            .volunteer_age_range(AgeRange::R18_24)
            .university(vec!["University of Moscow".to_owned()])
            .lgbt(Lgbt::No)
            .country("Russia".to_owned())
            .fli(vec![Fli::FirstGeneration])
            .student_stage(StudentStage::Freshman)
            .majors(vec!["Business Administration".to_owned()])
            .minors(vec!["Economics".to_owned()])
            .hear_about(vec![VolunteerHearAbout::University])
            .build()?,
    ];

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    let data = storage.batch_create_volunteers(project_cycle_id, data, &mut exec_opts).await?;

    dbg!(&data);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_volunteers(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    let volunteers = storage.fetch_volunteers(&mut exec_opts).await?;

    dbg!(&volunteers);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_volunteer_by_id(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let test_volunteer_id = uuid!("5e7b3f35-2b84-46e7-8b7b-73e2716d42c9");

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    let volunteer = storage.fetch_volunteer_by_id(test_volunteer_id, &mut exec_opts).await?;

    dbg!(&volunteer);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_volunteer_by_email(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let test_volunteer_email = "novak.djokovic@gmail.com";

    let mut exec_opts = ExecOptsBuilder::default().build()?;

    let volunteer = storage.fetch_volunteer_by_email(test_volunteer_email, &mut exec_opts).await?;

    dbg!(&volunteer);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn edit_volunteer(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let test_volunteer_id = uuid!("5e7b3f35-2b84-46e7-8b7b-73e2716d42c9");
    let data = EditVolunteerBuilder::default().email("novak.djokovic@gmail.com").build()?;

    let mut tx = storage.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;
    storage.edit_volunteer(test_volunteer_id, data, &mut exec_opts).await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_link_volunteers_to_nonprofits(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };

    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");

    let volunteer_id1 = uuid!("0ef67e25-543c-4f0d-9a96-8cb71b3c0f60");
    let volunteer_id2 = uuid!("fc32f485-4497-4ee9-a162-a5d12d6e737f");

    let client_id1 = uuid!("bb9b7fa5-7283-4b73-82e1-c7244e47421d");
    let client_id2 = uuid!("dce8d3c3-8f2d-4f3e-80d3-c1633f5282e2");

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    storage
        .batch_link_volunteers_to_nonprofits(
            project_cycle_id,
            vec![(volunteer_id1, client_id1), (volunteer_id2, client_id2)],
            &mut exec_opts,
        )
        .await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_link_volunteers_to_mentors(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };

    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");

    let volunteer_id1 = uuid!("0ef67e25-543c-4f0d-9a96-8cb71b3c0f60");
    let volunteer_id2 = uuid!("fc32f485-4497-4ee9-a162-a5d12d6e737f");
    let volunteer_id3 = uuid!("61596073-faf9-4cb5-b613-aa6b0ab86bb2");

    let mentor_id1 = uuid!("fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d");
    let mentor_id2 = uuid!("ab839d88-80c5-47f8-835a-1abbe269c7f8");

    let linkage =
        vec![(volunteer_id1, mentor_id1), (volunteer_id2, mentor_id1), (volunteer_id3, mentor_id2)];

    let mut exec_opts = ExecOptsBuilder::default().build()?;

    storage.batch_link_volunteers_to_mentors(project_cycle_id, linkage, &mut exec_opts).await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_batch_insert_and_remove_volunteers_exported_to_workspace(
    pool: PgPool,
) -> Result<()> {
    let storage = PgBackend { pool };

    let job_id = uuid!("413eed73-3c6f-456a-b9f0-ae72d136c742");

    let volunteer_id1 = uuid!("9edc52d8-8cc7-4d44-80c1-7efcce246e90");
    let workspace_email1 = "rogerfederer@developforgood.org";
    let org_unit = "/Programs/PantheonUsers";

    let volunteer_id2 = uuid!("1b1b5e16-d0d6-4ad1-8fdc-80df15b18b67");
    let workspace_email2 = "rafanadal@developforgood.org";

    let data = vec![
        InsertVolunteerExportedToWorkspaceBuilder::default()
            .job_id(job_id)
            .volunteer_id(volunteer_id1)
            .workspace_email(workspace_email1)
            .org_unit(org_unit)
            .build()?,
        InsertVolunteerExportedToWorkspaceBuilder::default()
            .job_id(job_id)
            .volunteer_id(volunteer_id2)
            .workspace_email(workspace_email2)
            .org_unit(org_unit)
            .build()?,
    ];

    let mut exec_opts = ExecOptsBuilder::default().build()?;
    storage.batch_insert_volunteers_exported_to_workspace(data, &mut exec_opts).await?;
    storage
        .batch_remove_volunteers_exported_to_workspace(
            vec![volunteer_id1, volunteer_id2],
            &mut exec_opts,
        )
        .await?;

    Ok(())
}
