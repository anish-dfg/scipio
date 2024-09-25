//! This module defines entities in Pantheon's database.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

use super::types::{
    AgeRange, ClientSize, Ethnicity, Fli, Gender, ImpactCause, JobStatus, Lgbt,
    MentorExperienceLevel, MentorYearsExperience, StudentStage, VolunteerHearAbout,
};

/// How a project cycle is represented in the database.
///
/// * `id`: The id of the project cycle
/// * `created_at`: The time the project cycle was created
/// * `updated_at`: The time the project cycle was last updated, if it was ever updated
/// * `name`: The name of the project cycle
/// * `description`: The description of the project cycle, if it exists
/// * `archived`: Whether the project cycle is archived
#[derive(FromRow, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCycle {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
}

/// How a volunteer is represented in the database.
///
/// * `id`: The id of the volunteer
/// * `created_at`: When the volunteer was created
/// * `updated_at`: When the volunteer was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the volunteer is associated with
/// * `first_name`: The volunteer's first name
/// * `last_name`: The volunteer's last name
/// * `email`: The volunteer's email address
/// * `phone`: The volunteer's phone number
/// * `offer_letter_signature`: Whether the volunteer has signed their offer letter
/// * `volunteer_gender`: The volunteer's gender
/// * `volunteer_ethnicity`: The volunteer's ethnicity
/// * `volunteer_age_range`: The volunteer's age range
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Volunteer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub volunteer_gender: Gender,
    pub volunteer_ethnicity: Ethnicity,
    pub volunteer_age_range: AgeRange,
}

/// VolunteerDetails corresponds to the `volunteer_details` view.
///
/// * `id`: The id of the volunteer
/// * `created_at`: When the volunteer was created
/// * `updated_at`: When the volunteer was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the volunteer is associated with
/// * `first_name`: The volunteer's first name
/// * `last_name`: The volunteer's last name
/// * `email`: The volunteer's email address
/// * `phone`: The volunteer's phone number
/// * `offer_letter_signature`: Whether the volunteer has signed their offer letter
/// * `volunteer_gender`: The volunteer's gender
/// * `volunteer_ethnicity`: The volunteer's ethnicity
/// * `volunteer_age_range`: The volunteer's age range
/// * `workspace_email`: The volunteer's workspace email (@developforgood.org)
/// * `clients`: The clients the volunteer is associated with
/// * `mentors`: The mentors the volunteer is associated with
/// * `roles`: The roles the volunteer has on their project team. These are not authentication
///   roles.
///
/// It's derived from a combination of the following relations in the database:
///
/// * `volunteers`
/// * `client_volunteers`
/// * `nonprofit_clients`
/// * `volunteer_mentors`
/// * `volunteer_team_roles`
/// * `team_roles`
/// * `project_cycles`
/// * `volunteers_exported_to_workspace`
///
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct VolunteerDetails {
    #[serde(rename = "id")]
    pub volunteer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub project_cycle_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub volunteer_gender: Gender,
    pub volunteer_ethnicity: Vec<Ethnicity>,
    pub volunteer_age_range: AgeRange,
    pub workspace_email: Option<String>,
    pub university: Vec<String>,
    pub lgbt: Lgbt,
    pub country: String,
    pub us_state: Option<String>,
    pub fli: Vec<Fli>,
    pub student_stage: StudentStage,
    pub majors: Vec<String>,
    pub minors: Vec<String>,
    pub hear_about: Vec<VolunteerHearAbout>,

    // university,
    // lgbt,
    // country,
    // us_state,
    // fli,
    // student_stage,
    // majors,
    // minors,
    // hear_about,

    // the following are all arrays but postgres returns them as json arrays so they can't be
    // represented as a Vec<T>
    pub clients: Value,
    pub mentors: Value,
    pub roles: Value,
}

/// NonprofitClientDetails corresponds to the `volunteer_details` view.
///
/// * `client_id`: The id of the nonprofit client
/// * `created_at`: When the nonprofit client was created
/// * `updated_at`: When the nonprofit client was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the nonprofit client is associated with
/// * `project_cycle_name`: The name of the project cycle the nonprofit client is associated with
/// * `representative_first_name`: The nonprofit client's representative's first name
/// * `representative_last_name`: The nonprofit client's representative's last name
/// * `representative_job_title`: The nonprofit client's representative's job title
/// * `email`: The nonprofit client's email address
/// * `email_cc`: The nonprofit client's email cc address
/// * `phone`: The nonprofit client's phone number
/// * `org_name`: The nonprofit client's organization name
/// * `project_name`: The nonprofit client's project name this cycle
/// * `org_website`: The nonprofit client's website
/// * `country_hq`: The country where the nonprofit client is headquartered
/// * `state_hq`: The state where the nonprofit client is headquartered
/// * `address`: The nonprofit client's address
/// * `size`: The size of the nonprofit client
/// * `volunteers`: The volunteers associated with the nonprofit client. This is their project team
/// * `mentors`: The mentors associated with the nonprofit client
#[derive(FromRow, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NonprofitClientDetails {
    pub client_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub project_cycle_name: String,
    pub representative_first_name: String,
    pub representative_last_name: String,
    pub representative_job_title: Option<String>,
    pub email: String,
    pub email_cc: Option<String>,
    pub phone: String,
    pub org_name: String,
    pub project_name: String,
    pub org_website: Option<String>,
    pub country_hq: Option<String>,
    pub us_state_hq: Option<String>,
    pub address: String,
    pub size: ClientSize,
    pub impact_causes: Vec<ImpactCause>,
    pub volunteers: Value,
    pub mentors: Value,
}

/// How a team role is represented in the database.
///
/// * `id`: The id of the team role
/// * `created_at`: The time the team role was created
/// * `updated_at`: The time the team role was last updated, if it was ever updated
/// * `name`: The name of the team role
/// * `description`: The description of the team role, if it exists
// NOTE: All of the team roles present in the database are hard coded based on the project roles in
// Airtable. More can be added if Develop for Good adds roles. If this becomes common, we can add
// an admin endpoint to add roles.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamRole {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub name: String,
    pub description: Option<String>,
}

/// How a mentor is represented in the database.
///
/// * `id`: The id of the mentor
/// * `created_at`: When the mentor was created
/// * `updated_at`: When the mentor was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the mentor is associated with
/// * `first_name`: The first_name of the mentor
/// * `last_name`: The last_name of the mentor
/// * `email`: The email of the mentor
/// * `phone`: The phone number of the mentor
/// * `offer_letter_signature`: Whether the mentor has signed their offer letter
/// * `company`: The company the mentor works for
/// * `job_title`: The job title of the mentor at their company
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mentor {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: String,
    pub job_title: String,
    pub country: String,
    pub us_state: Option<String>,
    pub years_experience: MentorYearsExperience,
    pub experience_level: MentorExperienceLevel,
    pub prior_mentor: bool,
    pub prior_mentee: bool,
    pub prior_student: bool,
    pub university: Vec<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
}

/// How a nonprofit client is represented in the database
///
/// * `id`: The id of the nonprofit client
/// * `created_at`: When the nonprofit client was created
/// * `updated_at`: When the nonprofit client was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the nonprofit client is associated with
/// * `project_cycle_name`: The name of the project cycle the nonprofit client is associated with
/// * `representative_first_name`: The nonprofit client's representative's first name
/// * `representative_last_name`: The nonprofit client's representative's last name
/// * `representative_job_title`: The nonprofit client's representative's job title
/// * `email`: The nonprofit client's email address
/// * `email_cc`: The nonprofit client's email cc address
/// * `phone`: The nonprofit client's phone number
/// * `org_name`: The nonprofit client's organization name
/// * `project_name`: The nonprofit client's project name this cycle
/// * `org_website`: The nonprofit client's website
/// * `country_hq`: The country where the nonprofit client is headquartered
/// * `state_hq`: The state where the nonprofit client is headquartered
/// * `address`: The nonprofit client's address
/// * `size`: The size of the nonprofit client
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NonprofitClient {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub representative_first_name: String,
    pub representative_last_name: String,
    pub representative_job_title: String,
    pub email: String,
    pub email_cc: Option<String>,
    pub phone: String,
    pub org_name: String,
    pub project_name: String,
    pub org_website: Option<String>,
    pub country_hq: Option<String>,
    pub us_state_hq: Option<String>,
    pub address: String,
    pub size: ClientSize,
    pub impact_causes: Vec<ImpactCause>,
}

/// How an asynchronous job is represented in the database.
///
/// * `id`: The id of the job
/// * `created_at`: When the job was created
/// * `updated_at`: When the job was last updated, if it was ever updated
/// * `project_cycle_id`: The id of project cycle this job is associated with, if it is associated
///    with a project cycle.
/// * `status`: The status of the job
/// * `label`: A friendly label for the job
/// * `description`: A friendly description of the job, if it exists
/// * `details`: Details about the job, stored as a JSON object. The `types` module contains more
///   information about the possible values of this field.
#[derive(FromRow, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Option<Uuid>,
    pub status: JobStatus,
    pub label: String,
    pub description: Option<String>,
    pub details: Value,
}

/// How a `mentor_details` view is represented in the database.
///
/// * `mentor_id`: The id of the mentor
/// * `created_at`: When the mentor was created
/// * `updated_at`: When the mentor was last updated, if it was ever updated
/// * `project_cycle_id`: The id of the project cycle the mentor is associated with
/// * `first_name`: The first_name of the mentor
/// * `last_name`: The last_name of the mentor
/// * `email`: The email of the mentor
/// * `phone`: The phone number of the mentor
/// * `offer_letter_signature`: Whether the mentor has signed their offer letter
/// * `company`: The company the mentor works for
/// * `job_title`: The job title of the mentor at their company
/// * `volunteers`: The volunteers the mentor is associated with
/// * `clients`: The cli ents the mentor is associated with
#[derive(FromRow, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MentorDetails {
    pub mentor_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub project_cycle_id: Uuid,
    pub project_cycle_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: String,
    pub job_title: String,
    pub country: String,
    pub us_state: Option<String>,
    pub years_experience: MentorYearsExperience,
    pub experience_level: MentorExperienceLevel,
    pub prior_mentor: bool,
    pub prior_mentee: bool,
    pub prior_student: bool,
    pub university: Vec<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
    pub volunteers: Value,
    pub clients: Value,
}

/// Basic stats about a project cycle
///
/// * `num_volunteers`: The number of volunteers in the project cycle
/// * `num_nonprofits`: The number of nonprofits in the project cycle
/// * `num_mentors`: The number of mentors in the project cycle
#[derive(FromRow, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BasicStats {
    pub num_volunteers: i64,
    pub num_nonprofits: i64,
    pub num_mentors: i64,
}
