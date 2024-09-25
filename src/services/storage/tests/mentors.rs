use anyhow::Result;
use sqlx::PgPool;
use uuid::uuid;

use crate::services::storage::mentors::{CreateMentorBuilder, EditMentorBuilder, QueryMentors};
use crate::services::storage::types::{
    MentorExperienceLevel, MentorYearsExperience, VolunteerHearAbout,
};
use crate::services::storage::{ExecOptsBuilder, PgBackend};

#[sqlx::test(fixtures("setup"))]
pub async fn test_create_mentor(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");
    let data = CreateMentorBuilder::default()
        .first_name("Martina")
        .last_name("Navratilova")
        .email("martina.navratilova@gmail.com")
        .phone("777-888-9999")
        .company("Tennis Channel")
        .job_title("Commentator")
        .country("United States".to_string())
        .us_state("California".to_string())
        .years_experience(MentorYearsExperience::R21Plus)
        .experience_level(MentorExperienceLevel::SeniorOrExecutive)
        .prior_mentor(true)
        .prior_mentee(true)
        .prior_student(true)
        .university(vec!["University of Prague".to_string()])
        .hear_about(vec![VolunteerHearAbout::OnlineAd])
        .build()?;

    let id = storage
        .create_mentor(project_cycle_id, data, &mut ExecOptsBuilder::default().build()?)
        .await?;

    dbg!(id);

    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_batch_create_mentors(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let project_cycle_id = uuid!("0e12b846-4de5-432e-8137-1bc2c92827b3");
    let data = vec![
        CreateMentorBuilder::default()
            .first_name("Chris")
            .last_name("Evert")
            .email("chris.evert@gmail.com")
            .phone("313-313-1313")
            .company("Tennis Channel")
            .job_title("Commentator")
            .country("United States".to_string())
            .us_state("California".to_string())
            .years_experience(MentorYearsExperience::R21Plus)
            .experience_level(MentorExperienceLevel::SeniorOrExecutive)
            .prior_mentor(true)
            .prior_mentee(true)
            .prior_student(true)
            .university(vec!["Stanford University".to_string()])
            .hear_about(vec![VolunteerHearAbout::OnlineAd])
            .build()?,
        CreateMentorBuilder::default()
            .first_name("Steffi")
            .last_name("Graf")
            .email("steffi.graf@gmail.com")
            .phone("414-141-4141")
            .company("Tennis Channel")
            .job_title("Commentator")
            .country("Germany".to_string())
            .us_state("Westphalia".to_string())
            .years_experience(MentorYearsExperience::R21Plus)
            .experience_level(MentorExperienceLevel::SeniorOrExecutive)
            .prior_mentor(true)
            .prior_mentee(true)
            .prior_student(true)
            .university(vec!["University of Saarland".to_string()])
            .hear_about(vec![VolunteerHearAbout::OnlineAd])
            .build()?,
    ];

    let mentors = storage
        .batch_create_mentors(project_cycle_id, data, &mut ExecOptsBuilder::default().build()?)
        .await?;

    dbg!(&mentors);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_mentors(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let mentors = storage.fetch_mentors(&mut ExecOptsBuilder::default().build()?).await?;
    dbg!(mentors);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_fetch_mentor_by_id(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let mentor_id = uuid!("fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d");
    let mentor =
        storage.fetch_mentor_by_id(mentor_id, &mut ExecOptsBuilder::default().build()?).await?;
    dbg!(mentor);
    Ok(())
}

#[sqlx::test(fixtures("setup"))]
pub async fn test_edit_mentor(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let mentor_id = uuid!("fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d");
    let data = EditMentorBuilder::default()
        .first_name("John")
        .last_name("McEnroe")
        .email("johnny.mac@gmail.com")
        .phone("191-191-1887")
        .build()?;
    storage.edit_mentor(mentor_id, data, &mut ExecOptsBuilder::default().build()?).await?;

    Ok(())
}

#[sqlx::test(fixtures("setup"))]

pub async fn test_delete_mentor(pool: PgPool) -> Result<()> {
    let storage = PgBackend { pool };
    let mentor_id = uuid!("fa8377c8-1c0d-4f4e-9a2b-2c29f2737e0d");
    storage.delete_mentor(mentor_id, &mut ExecOptsBuilder::default().build()?).await?;
    Ok(())
}
