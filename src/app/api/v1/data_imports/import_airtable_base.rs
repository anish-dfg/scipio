//  ╭─────────────────────────────────────────────────────────╮
//  │              Importing an Airtable base                 │
//  ╰─────────────────────────────────────────────────────────╯
//   ─────────────────────────────────────────────────────────
//   @author Anish Sinha <anish@developforgood.org>
//   ─────────────────────────────────────────────────────────
//   This file implements importing an airtable base according
//   to the version 1 schema.
//

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_nats::Subscriber;
use derive_builder::Builder;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::time;
use uuid::Uuid;

use crate::app::context::Context;
use crate::services::airtable::base_data::records::responses::ListRecordsResponse;
use crate::services::airtable::base_data::records::ListRecordsQueryBuilder;
use crate::services::airtable::entities::record::Record;
use crate::services::storage::cycles::CreateCycleBuilder;
use crate::services::storage::jobs::UpdateJobStatus;
use crate::services::storage::mentors::CreateMentor;
use crate::services::storage::nonprofits::CreateNonprofit;
use crate::services::storage::types::{
    AgeRange, ClientSize, Ethnicity, Fli, Gender, ImpactCause, JobStatus, Lgbt,
    MentorExperienceLevel, MentorYearsExperience, StudentStage, VolunteerHearAbout,
};
use crate::services::storage::volunteers::CreateVolunteer;
use crate::services::storage::ExecOptsBuilder;

/// This fetches nonprofits from Develop for Good's Airtable.
///
/// * `ctx`: Copy of app context
/// * `base_id`: The base to fetch from
///
/// TODO: Refactor the specific view name to a more generic one (Finalized Sum24 Nonprofit Projects -> Finalized Nonprofit Projects)
async fn fetch_nonprofits(ctx: Arc<Context>, base_id: &str) -> Result<Vec<Record<Value>>> {
    let airtable = &ctx.airtable;

    let mut query_opts = ListRecordsQueryBuilder::default()
        .fields(
            [
                "OrgName",
                "ProjectName",
                "OrgWebsite",
                "FirstName",
                "LastName",
                "JobTitle",
                "NonprofitEmail",
                "Phone",
                "CountryHQ",
                "StateHQ",
                "Address",
                "Size",
                "ImpactCauses",
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        )
        .view("Finalized Sum24 Nonprofit Projects".to_owned())
        .build()?;

    let mut nonprofits = Vec::<Record<Value>>::with_capacity(50);

    loop {
        let ListRecordsResponse { ref mut records, offset } =
            airtable.list_records(base_id, "Nonprofits", Some(query_opts.clone())).await?;

        nonprofits.append(records);

        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }

    Ok(nonprofits)
}

#[derive(Builder, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntermediateNonprofitData {
    #[builder(setter(into))]
    #[serde(rename = "FirstName")]
    pub representative_first_name: String,
    #[builder(setter(into))]
    #[serde(rename = "LastName")]
    pub representative_last_name: String,
    #[builder(setter(into))]
    #[serde(rename = "JobTitle")]
    pub representative_job_title: String,
    #[builder(setter(into))]
    #[serde(rename = "NonprofitEmail")]
    pub email: String,
    #[serde(rename = "Cc")]
    #[builder(setter(into), default = "None")]
    pub email_cc: Option<String>,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub org_name: String,
    #[builder(setter(into))]
    pub project_name: String,
    #[builder(setter(into), default = "None")]
    pub org_website: Option<String>,
    #[serde(rename = "CountryHQ")]
    #[builder(setter(into), default = "None")]
    pub country_hq: Option<String>,
    #[serde(rename = "StateHQ")]
    #[builder(setter(into), default = "None")]
    pub us_state_hq: Option<String>,
    #[builder(setter(into))]
    pub address: String,
    pub size: ClientSize,
    pub impact_causes: Option<Vec<String>>,
}

impl From<IntermediateNonprofitData> for CreateNonprofit {
    fn from(value: IntermediateNonprofitData) -> Self {
        CreateNonprofit {
            representative_first_name: value.representative_first_name,
            representative_last_name: value.representative_last_name,
            representative_job_title: value.representative_job_title,
            email: value.email,
            email_cc: value.email_cc,
            phone: value.phone,
            org_name: value.org_name,
            project_name: value.project_name,
            org_website: value.org_website,
            country_hq: value.country_hq,
            us_state_hq: value.us_state_hq,
            address: value.address,
            size: value.size,
            impact_causes: value
                .impact_causes
                .unwrap_or(vec!["Other".to_owned()])
                .iter()
                .map(|c| match c.as_str() {
                    "reco1zHRYv8lTQDaI" => ImpactCause::Animals,
                    "recXhhTPsuQ2PMjU4" => ImpactCause::CareerAndProfessionalDevelopment,
                    "recvWKilRRABCcHuI" => ImpactCause::DisasterRelief,
                    "recYfRNFDpm2nedjM" => ImpactCause::Education,
                    "recOlWiJTppnQwnll" => ImpactCause::EnvironmentAndSustainability,
                    "recix0Y5qCXYfZGRz" => ImpactCause::FaithAndReligion,
                    "recKs8kboTORruStC" => ImpactCause::HealthAndMedicine,
                    "recEmtYMgeOlPeOVQ" => ImpactCause::GlobalRelations,
                    "reczSSbvdW2NoOX2p" => ImpactCause::PovertyAndHunger,
                    "rec5dt6EVyUeIaCR7" => ImpactCause::SeniorServices,
                    "recMt9349gwuRAQXf" => ImpactCause::JusticeAndEquity,
                    "rec8cH6YTQMeYqXUh" => ImpactCause::VeteransAndMilitaryFamilies,
                    _ => ImpactCause::Other,
                })
                .collect(),
        }
    }
}

// reco1zHRYv8lTQDaI

fn process_raw_nonprofits(nonprofits: Vec<Record<Value>>) -> Result<Vec<CreateNonprofit>> {
    let create_nonprofit_data = nonprofits
        .iter()
        .filter_map(|n| {
            match serde_json::from_value::<IntermediateNonprofitData>(n.fields.clone()) {
                Ok(data) => Some(data),
                Err(e) => {
                    log::error!("{}", e);
                    None
                }
            }
        })
        .map(CreateNonprofit::from)
        .collect::<Vec<_>>();

    log::info!("Processed {} nonprofits", create_nonprofit_data.len());
    Ok(create_nonprofit_data)
}

/// This fetches volunteers from Develop for Good's Airtable.
///
/// * `ctx`: Copy of app context
/// * `base_id`: The base to fetch from
async fn fetch_volunteers(ctx: Arc<Context>, base_id: &str) -> Result<Vec<Record<Value>>> {
    let airtable = &ctx.airtable;

    let fields = [
        "FirstName",
        "LastName",
        "OrgName (from ProjectRecordID)",
        "Email",
        "Phone",
        "Gender",
        "Ethnicity",
        "AgeRange",
        "University",
        "LGBT",
        "Country",
        "State",
        "FLI",
        "StudentStage",
        "Majors",
        "Minors",
        "HearAbout",
    ]
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>();

    let mut query_opts = ListRecordsQueryBuilder::default()
        .fields(fields)
        .view("All Committed Student Volunteers - Active".to_owned())
        .build()?;

    let mut volunteers = Vec::<Record<Value>>::with_capacity(500);

    loop {
        let ListRecordsResponse { ref mut records, offset } =
            airtable.list_records(base_id, "Volunteers", Some(query_opts.clone())).await?;
        volunteers.append(records);
        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }
    Ok(volunteers)
}

/// Container for holding processed volunteer data.
///
/// * `volunteers`: Data for creating volunteers as records in the database
/// * `linkage`: The linkage between volunteers and nonprofits
struct ProcessedVolunteers {
    create_volunteer_data: Vec<CreateVolunteer>,
    volunteer_nonprofit_linkage: Vec<(String, String)>,
}

#[derive(Debug, Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct IntermediateVolunteerData {
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into), default = "None")]
    pub phone: Option<String>,
    #[builder(default = "Gender::PreferNotToSay")]
    #[serde(rename = "Gender")]
    pub volunteer_gender: Gender,
    #[builder(default = "vec![Ethnicity::PreferNotToSay]")]
    #[serde(rename = "Ethnicity")]
    pub volunteer_ethnicity: Vec<Ethnicity>,
    #[builder(default = "AgeRange::R18_24")]
    #[serde(rename = "AgeRange")]
    pub volunteer_age_range: AgeRange,
    #[builder(default = "vec![]")]
    pub university: Vec<String>,
    #[serde(rename = "LGBT")]
    pub lgbt: Lgbt,
    pub country: String,
    #[builder(setter(into), default = "None")]
    #[serde(rename = "State")]
    pub us_state: Option<String>,
    #[builder(default = "vec![Fli::PreferNotToSay]")]
    #[serde(rename = "FLI")]
    pub fli: Vec<Fli>,
    pub student_stage: StudentStage,
    pub majors: String,
    pub minors: Option<String>,
    pub hear_about: Vec<VolunteerHearAbout>,
}

impl From<IntermediateVolunteerData> for CreateVolunteer {
    fn from(value: IntermediateVolunteerData) -> Self {
        CreateVolunteer {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            volunteer_gender: value.volunteer_gender,
            volunteer_ethnicity: value.volunteer_ethnicity,
            volunteer_age_range: value.volunteer_age_range,
            university: value.university,
            lgbt: value.lgbt,
            country: value.country,
            us_state: value.us_state,
            fli: value.fli,
            student_stage: value.student_stage,
            majors: value.majors.split(",").map(|m| m.trim().to_string()).collect(),
            minors: value
                .minors
                .unwrap_or_default()
                .split(",")
                .map(|m| m.trim().to_string())
                .collect(),
            hear_about: value.hear_about,
        }
    }
}

/// Processes the raw volunteer data from Airtable (received as records) and retrieves information
/// to create volunteers in the database as well as linkage information between volunteers and
/// nonprofits
///
/// * `volunteers`: The raw volunteer data
fn process_raw_volunteers(volunteers: Vec<Record<Value>>) -> Result<ProcessedVolunteers> {
    let mut volunteer_nonprofit_linkage = Vec::<(String, String)>::with_capacity(500);

    let create_volunteer_data = volunteers
        .iter()
        .filter_map(|v| {
            match (
                &v.fields["OrgName (from ProjectRecordID)"],
                serde_json::from_value::<IntermediateVolunteerData>(v.fields.clone()),
            ) {
                (Value::Array(clients), Ok(data)) => {
                    for client in clients {
                        if let Some(org_name) = client.as_str() {
                            volunteer_nonprofit_linkage
                                .push((data.email.clone(), org_name.to_owned()));
                        }
                    }
                    Some(data)
                }
                (_, d) => {
                    dbg!(&d);
                    None
                }
            }
        })
        .map(CreateVolunteer::from)
        .collect::<Vec<_>>();

    log::info!("Processed {} mentors", create_volunteer_data.len());
    Ok(ProcessedVolunteers { create_volunteer_data, volunteer_nonprofit_linkage })
}

/// This fetches mentors from Develop for Good's Airtable.
///
/// * `ctx`: Copy of the app context
/// * `base_id`: The base to fetch from
async fn fetch_mentors(ctx: Arc<Context>, base_id: &str) -> Result<Vec<Record<Value>>> {
    let airtable = &ctx.airtable;

    let fields = [
        "FirstName",
        "LastName",
        "Email",
        "Phone",
        "OfferLetterSignature",
        "Company",
        "JobTitle",
        "ProjectRole",
        "OrgName (from ProjectRecordID)",
        "Country",
        "State",
        "YearsExperience",
        "ExperienceLevel",
        "PriorMentorship",
        "PriorDFG",
        "University",
        "HearAbout",
    ]
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>();

    let mut query_opts = ListRecordsQueryBuilder::default()
        .fields(fields)
        .view("All Committed Mentor Volunteers".to_owned())
        .build()?;

    let mut mentors = Vec::<Record<Value>>::with_capacity(50);

    loop {
        let ListRecordsResponse { ref mut records, offset } =
            airtable.list_records(base_id, "Volunteers", Some(query_opts.clone())).await?;
        mentors.append(records);
        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }

    Ok(mentors)
}

///  Container for holding processed mentor data.
///
/// * `mentors`: Data for creating mentors as records in the database
/// * `linkage`: The linkage between mentors and nonprofits
struct ProcessedMentors {
    create_mentor_data: Vec<CreateMentor>,
    mentor_nonprofit_linkage: Vec<(String, Vec<String>)>,
}

#[derive(Builder, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct IntermediateMentorData {
    // pub project_cycle_id: Uuid,
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    pub phone: String,
    #[builder(setter(into))]
    pub company: String,
    #[builder(setter(into))]
    pub job_title: String,
    #[builder(setter(into))]
    pub country: String,
    #[builder(setter(into), default = "None")]
    pub us_state: Option<String>,
    #[builder(setter(into))]
    pub years_experience: String,
    #[builder(setter(into))]
    pub experience_level: MentorExperienceLevel,
    pub prior_mentorship: Vec<String>,
    #[serde(rename = "PriorDFG")]
    pub prior_dfg: Option<Vec<String>>,
    pub university: Option<Vec<String>>,
    pub hear_about: Option<Vec<VolunteerHearAbout>>,
}

// impl TryFrom<IntermediateMentorData> for CreateMentor {
//     type Error = anyhow::Error;
//
//     fn try_from(value: IntermediateMentorData) -> std::result::Result<Self, Self::Error> {
//
//     }
// }

impl From<IntermediateMentorData> for CreateMentor {
    fn from(value: IntermediateMentorData) -> Self {
        CreateMentor {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            phone: value.phone,
            company: value.company,
            job_title: value.job_title,
            country: value.country,
            us_state: value.us_state,
            years_experience: match value.years_experience.as_str() {
                "2-5" => MentorYearsExperience::R2_5,
                "6-10" => MentorYearsExperience::R6_10,
                "11-15" => MentorYearsExperience::R11_15,
                "16-20" => MentorYearsExperience::R16_20,
                _ => MentorYearsExperience::R21Plus,
            },
            experience_level: value.experience_level,
            prior_mentor: value.prior_mentorship.contains(&"Yes, I've been a mentor".to_owned()),
            prior_mentee: value.prior_mentorship.contains(&"Yes, I've been a mentee".to_owned()),
            // prior_student: !value.prior_dfg.contains(&"No".to_owned()),
            prior_student: value.prior_dfg.unwrap_or_default().contains(&"Yes".to_owned()),
            university: value.university.unwrap_or_default(),
            hear_about: value.hear_about.unwrap_or(vec![VolunteerHearAbout::Other]),
        }
    }
}

/// Processes the raw mentor data from Airtable (received as records) and retrieves information
/// to create mentors in the database as well as linkage information between mentors and
/// nonprofits
///
/// * `mentors`: The raw mentor data
fn process_raw_mentors(mentors: Vec<Record<Value>>) -> Result<ProcessedMentors> {
    let mut mentor_nonprofit_linkage = Vec::<(String, Vec<String>)>::with_capacity(50);

    mentors.iter().for_each(|mentor| {
        if let (Some(email), Some(Value::Array(clients)), Some(Value::Array(project_roles))) = (
            mentor.fields.get("Email").and_then(|v| v.as_str().map(String::from)),
            mentor.fields.get("OrgName (from ProjectRecordID)"),
            mentor.fields.get("ProjectRole"),
        ) {
            let nonprofit_names = clients
                .iter()
                .filter_map(|project| project.as_str().map(String::from))
                .collect::<Vec<String>>();

            let role_names = project_roles
                .iter()
                .filter_map(|role| role.as_str().map(String::from))
                .collect::<Vec<String>>();

            if role_names.contains(&"Team Mentor".to_owned()) {
                mentor_nonprofit_linkage.push((email, nonprofit_names));
            }
        }
    });

    let create_mentor_data = mentors
        .iter()
        .filter_map(|m| match serde_json::from_value::<IntermediateMentorData>(m.fields.clone()) {
            Ok(data) => Some(data),
            Err(e) => {
                log::error!("{m:?}");
                log::error!("{}", e);
                None
            }
        })
        .map(CreateMentor::from)
        .collect::<Vec<_>>();

    log::info!("Processed {} mentors", create_mentor_data.len());
    Ok(ProcessedMentors { create_mentor_data, mentor_nonprofit_linkage })
}

async fn fetch_mentor_mentee_linkage(
    ctx: Arc<Context>,
    base_id: &str,
) -> Result<Vec<(String, String)>> {
    let mut query_opts = ListRecordsQueryBuilder::default()
        .fields(vec!["Email".to_owned(), "Mentee Email (from Volunteers)".to_owned()])
        .view("All Committed Mentor Volunteers - 1:1 Mentor-Mentee Pairings".to_owned())
        .build()?;
    let mut mentor_mentee_linkage = Vec::<(String, String)>::with_capacity(50);

    loop {
        let ListRecordsResponse { ref mut records, offset } =
            ctx.airtable.list_records(base_id, "Volunteers", Some(query_opts.clone())).await?;

        for record in records.iter() {
            if let (Some(email), Some(Value::Array(mentee_emails))) = (
                record.fields.get("Email").and_then(|v| v.as_str().map(String::from)),
                record.fields.get("Mentee Email (from Volunteers)"),
            ) {
                mentee_emails.iter().for_each(|mentee_email| {
                    if let Some(mentee_email) = mentee_email.as_str() {
                        mentor_mentee_linkage.push((email.clone(), mentee_email.to_owned()));
                    }
                });
            }
        }

        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }
    Ok(mentor_mentee_linkage)
}

struct ImportBaseData {
    name: String,
    description: String,
    nonprofits: Vec<CreateNonprofit>,
    volunteers: Vec<CreateVolunteer>,
    volunteer_nonprofit_linkage: Vec<(String, String)>,
    mentors: Vec<CreateMentor>,
    mentor_nonprofit_linkage: Vec<(String, Vec<String>)>,
    mentor_mentee_linkage: Vec<(String, String)>,
}

async fn collect_import_base_data(
    ctx: Arc<Context>,
    base_id: &str,
    name: &str,
    description: &str,
) -> Result<ImportBaseData> {
    let nonprofits = fetch_nonprofits(ctx.clone(), base_id).await?;
    let create_nonprofit_data = process_raw_nonprofits(nonprofits)?;

    let volunteers = fetch_volunteers(ctx.clone(), base_id).await?;
    let ProcessedVolunteers { create_volunteer_data, volunteer_nonprofit_linkage } =
        process_raw_volunteers(volunteers)?;

    let mentors = fetch_mentors(ctx.clone(), base_id).await?;
    let ProcessedMentors { create_mentor_data, mentor_nonprofit_linkage } =
        process_raw_mentors(mentors)?;

    let mentor_mentee_linkage = fetch_mentor_mentee_linkage(ctx.clone(), base_id).await?;

    Ok(ImportBaseData {
        name: name.to_owned(),
        description: description.to_owned(),
        nonprofits: create_nonprofit_data,
        volunteers: create_volunteer_data,
        volunteer_nonprofit_linkage,
        mentors: create_mentor_data,
        mentor_nonprofit_linkage,
        mentor_mentee_linkage,
    })
}

async fn store_base_data(ctx: Arc<Context>, data: ImportBaseData, job_id: Uuid) -> Result<()> {
    let storage_layer = &ctx.storage_layer;
    let mut tx = storage_layer.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;

    let project_cycle_id = storage_layer
        .create_cycle(
            CreateCycleBuilder::default().name(data.name).description(data.description).build()?,
            &mut exec_opts,
        )
        .await?;

    let nonprofits = storage_layer
        .batch_create_nonprofits(project_cycle_id, data.nonprofits, &mut exec_opts)
        .await?;

    let volunteers = storage_layer
        .batch_create_volunteers(project_cycle_id, data.volunteers, &mut exec_opts)
        .await?;

    let mentors =
        storage_layer.batch_create_mentors(project_cycle_id, data.mentors, &mut exec_opts).await?;

    let nonprofit_name_id_map = HashMap::<String, Uuid>::from_iter(nonprofits);
    let volunteer_email_id_map = HashMap::<String, Uuid>::from_iter(volunteers);
    let mentor_email_id_map = HashMap::<String, Uuid>::from_iter(mentors);

    let volunteer_nonprofit_linkage = data
        .volunteer_nonprofit_linkage
        .iter()
        .filter_map(|(email, org_name)| {
            let volunteer_id = volunteer_email_id_map.get(email)?;
            let nonprofit_id = nonprofit_name_id_map.get(org_name)?;
            Some((*volunteer_id, *nonprofit_id))
        })
        .collect::<Vec<(Uuid, Uuid)>>();

    let mentor_nonprofit_linkage = data
        .mentor_nonprofit_linkage
        .iter()
        .filter_map(|(email, org_names)| {
            let mentor_id = mentor_email_id_map.get(email)?;
            let nonprofit_ids = org_names
                .iter()
                .filter_map(|org_name| nonprofit_name_id_map.get(org_name))
                .cloned()
                .collect::<Vec<Uuid>>();
            Some((*mentor_id, nonprofit_ids))
        })
        .flat_map(|(mentor_id, nonprofit_ids)| {
            nonprofit_ids
                .iter()
                .map(|nonprofit_id| (mentor_id, *nonprofit_id))
                .collect::<Vec<(Uuid, Uuid)>>()
        })
        .collect::<Vec<(Uuid, Uuid)>>();

    let volunteer_mentee_linkage = data
        .mentor_mentee_linkage
        .iter()
        .filter_map(|(mentor_email, mentee_email)| {
            let mentor_id = mentor_email_id_map.get(mentor_email);
            let mentee_id = volunteer_email_id_map.get(mentee_email);
            if mentor_id.is_none() || mentee_id.is_none() {
                // log::info!(
                //     "mentor_email: {:?}, mentee_email: {:?}",
                //     mentor_email,
                //     mentee_email
                // );
            }
            Some((*(mentee_id?), *(mentor_id?)))
        })
        .collect::<Vec<(Uuid, Uuid)>>();

    if !volunteer_nonprofit_linkage.is_empty() {
        storage_layer
            .batch_link_volunteers_to_nonprofits(
                project_cycle_id,
                volunteer_nonprofit_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    if !mentor_nonprofit_linkage.is_empty() {
        storage_layer
            .batch_link_mentors_to_nonprofits(
                project_cycle_id,
                mentor_nonprofit_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    if !volunteer_mentee_linkage.is_empty() {
        storage_layer
            .batch_link_volunteers_to_mentors(
                project_cycle_id,
                volunteer_mentee_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    // Update this job's project_cycle_id now that the project cycle exists
    storage_layer.set_job_project_cycle(job_id, project_cycle_id, &mut exec_opts).await?;

    // let update_data = UpdateJobStatus { status: JobStatus::Complete, error: None };
    // storage_layer.update_job_status(job_id, update_data, &mut exec_opts).await?;

    tx.commit().await?;
    Ok(())
}

struct RunTaskParams {
    pub name: String,
    pub description: String,
    pub job_id: Uuid,
    pub base_id: String,
}

async fn run_task(ctx: Arc<Context>, params: RunTaskParams) -> Result<()> {
    // NOTE: Uncomment the following two lines to give yourself enough time to cancel the task
    // log::info!("Sleeping for 100 seconds");
    // time::sleep(Duration::from_secs(100)).await;

    let data =
        collect_import_base_data(ctx.clone(), &params.base_id, &params.name, &params.description)
            .await?;

    store_base_data(ctx.clone(), data, params.job_id).await?;
    Ok(())
}

pub(super) struct ImportTaskParams {
    pub name: String,
    pub description: String,
    pub job_id: Uuid,
    pub base_id: String,
    pub subscriber: Subscriber,
}

impl From<ImportTaskParams> for RunTaskParams {
    fn from(value: ImportTaskParams) -> Self {
        RunTaskParams {
            name: value.name,
            description: value.description,
            job_id: value.job_id,
            base_id: value.base_id,
        }
    }
}

///  This function runs a full import of an Airtable base asynchronously. It is cancellable at any
///  point before the data is fetched from airtable or at any point before
///  [run_task_with_cancellation] acquires a write lock on the cancellable tasks map. It will also
///  time out if the task takes more than two minutes to complete.
///
/// * `ctx`:  A copy of the app context
/// * `params`: The parameters for the import task [ImportTaskParams]
pub async fn import_task(ctx: Arc<Context>, mut params: ImportTaskParams) -> Result<()> {
    let storage_layer = &ctx.storage_layer;
    let job_id = params.job_id;

    let run_params = RunTaskParams {
        name: params.name.clone(),
        description: params.description.clone(),
        job_id,
        base_id: params.base_id.clone(),
    };

    tokio::select! {
        _ = params.subscriber.next() => {
            storage_layer.cancel_job(job_id, &mut ExecOptsBuilder::default().build()?).await?;
        }
        res = run_task(ctx.clone(), run_params) => {
            match res {
                Ok(_) => {
                    let data = UpdateJobStatus { status: JobStatus::Complete, error: None };
                    storage_layer.update_job_status(job_id, data, &mut ExecOptsBuilder::default().build()?).await?;
                },
                Err(e) => {
                    let data = UpdateJobStatus { status: JobStatus::Error, error: Some(e.to_string()) };
                    storage_layer.update_job_status(job_id, data, &mut ExecOptsBuilder::default().build()?).await?;
                },
            };
        }
        () = time::sleep(Duration::from_secs(600)) => {
            // NOTE: A request times out after 10 minutes. An export task really shouldn't take
            // this long.
            log::warn!("request timed out");
        }
    };

    Ok(())
}
