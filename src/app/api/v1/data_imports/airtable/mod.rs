mod intermediate;

use std::collections::HashMap;

use anyhow::{bail, Result};
use intermediate::{IntermediateMentorData, IntermediateNonprofitData, IntermediateVolunteerData};
use serde_json::Value;
use uuid::Uuid;

use super::ImportServices;
use crate::services::airtable::base_data::records::responses::ListRecordsResponse;
use crate::services::airtable::base_data::records::ListRecordsQueryBuilder;
use crate::services::airtable::entities::record::Record;
use crate::services::airtable::entities::schema::Table;
use crate::services::storage::cycles::CreateCycleBuilder;
use crate::services::storage::mentors::CreateMentor;
use crate::services::storage::nonprofits::CreateNonprofit;
use crate::services::storage::volunteers::CreateVolunteer;
use crate::services::storage::ExecOptsBuilder;

pub struct ImportParams {
    pub name: String,
    pub description: String,
    pub job_id: Uuid,
    pub base_id: String,
    pub tables: Vec<Table>,
}

async fn fetch_nonprofits(
    services: &ImportServices,
    params: &ImportParams,
) -> Result<Vec<Record<Value>>> {
    let Some(nonprofits_table) = params.tables.iter().find(|t| t.name == "Nonprofits") else {
        bail!("Nonprofits table not found");
    };

    let Some(finalized_nonprofits_view) = nonprofits_table
        .views
        .iter()
        .find(|t| t.name.starts_with("Finalized") && t.name.ends_with("Nonprofit Projects"))
    else {
        bail!("Finalized Nonprofits view not found: Name should start with `Finalized` and end with `Nonprofit Projects`");
    };

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
        .view(finalized_nonprofits_view.name.clone())
        .build()?;

    let mut nonprofits = Vec::<Record<Value>>::with_capacity(50);

    loop {
        let ListRecordsResponse { ref mut records, offset } = services
            .airtable
            .list_records(&params.base_id, "Nonprofits", Some(query_opts.clone()))
            .await?;

        nonprofits.append(records);

        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }

    Ok(nonprofits)
}

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

async fn fetch_volunteers(
    services: &ImportServices,
    params: &ImportParams,
) -> Result<Vec<Record<Value>>> {
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
        let ListRecordsResponse { ref mut records, offset } = services
            .airtable
            .list_records(&params.base_id, "Volunteers", Some(query_opts.clone()))
            .await?;
        volunteers.append(records);
        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }
    Ok(volunteers)
}

struct ProcessedVolunteers {
    create_volunteer_data: Vec<CreateVolunteer>,
    volunteer_nonprofit_linkage: Vec<(String, String)>,
}

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

    log::info!("Processed {} volunteers", create_volunteer_data.len());
    Ok(ProcessedVolunteers { create_volunteer_data, volunteer_nonprofit_linkage })
}

async fn fetch_mentors(
    services: &ImportServices,
    params: &ImportParams,
) -> Result<Vec<Record<Value>>> {
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
        let ListRecordsResponse { ref mut records, offset } = services
            .airtable
            .list_records(&params.base_id, "Volunteers", Some(query_opts.clone()))
            .await?;
        mentors.append(records);
        if let Some(new_offset) = offset {
            query_opts.offset = Some(new_offset);
        } else {
            break;
        }
    }

    Ok(mentors)
}

struct ProcessedMentors {
    create_mentor_data: Vec<CreateMentor>,
    mentor_nonprofit_linkage: Vec<(String, Vec<String>)>,
}

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
    services: &ImportServices,
    params: &ImportParams,
) -> Result<Vec<(String, String)>> {
    let mut query_opts = ListRecordsQueryBuilder::default()
        .fields(vec!["Email".to_owned(), "Mentee Email (from Volunteers)".to_owned()])
        .view("All Committed Mentor Volunteers - 1:1 Mentor-Mentee Pairings".to_owned())
        .build()?;
    let mut mentor_mentee_linkage = Vec::<(String, String)>::with_capacity(50);

    loop {
        let ListRecordsResponse { ref mut records, offset } = services
            .airtable
            .list_records(&params.base_id, "Volunteers", Some(query_opts.clone()))
            .await?;

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
    services: &ImportServices,
    params: &ImportParams,
) -> Result<ImportBaseData> {
    let nonprofits = fetch_nonprofits(services, params).await?;
    let create_nonprofit_data = process_raw_nonprofits(nonprofits)?;

    let volunteers = fetch_volunteers(services, params).await?;
    let ProcessedVolunteers { create_volunteer_data, volunteer_nonprofit_linkage } =
        process_raw_volunteers(volunteers)?;

    let mentors = fetch_mentors(services, params).await?;
    let ProcessedMentors { create_mentor_data, mentor_nonprofit_linkage } =
        process_raw_mentors(mentors)?;

    let mentor_mentee_linkage = fetch_mentor_mentee_linkage(services, params).await?;

    Ok(ImportBaseData {
        name: params.name.to_owned(),
        description: params.description.to_owned(),
        nonprofits: create_nonprofit_data,
        volunteers: create_volunteer_data,
        volunteer_nonprofit_linkage,
        mentors: create_mentor_data,
        mentor_nonprofit_linkage,
        mentor_mentee_linkage,
    })
}

async fn store_base_data(
    services: &ImportServices,
    data: ImportBaseData,
    params: &ImportParams,
) -> Result<()> {
    let mut tx = services.storage_layer.acquire().await?;
    let mut exec_opts = ExecOptsBuilder::default().tx(&mut tx).build()?;

    let project_cycle_id = services
        .storage_layer
        .create_cycle(
            CreateCycleBuilder::default().name(data.name).description(data.description).build()?,
            &mut exec_opts,
        )
        .await?;

    let nonprofits = services
        .storage_layer
        .batch_create_nonprofits(project_cycle_id, data.nonprofits, &mut exec_opts)
        .await?;

    let volunteers = services
        .storage_layer
        .batch_create_volunteers(project_cycle_id, data.volunteers, &mut exec_opts)
        .await?;

    let mentors = services
        .storage_layer
        .batch_create_mentors(project_cycle_id, data.mentors, &mut exec_opts)
        .await?;

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
        services
            .storage_layer
            .batch_link_volunteers_to_nonprofits(
                project_cycle_id,
                volunteer_nonprofit_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    if !mentor_nonprofit_linkage.is_empty() {
        services
            .storage_layer
            .batch_link_mentors_to_nonprofits(
                project_cycle_id,
                mentor_nonprofit_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    if !volunteer_mentee_linkage.is_empty() {
        services
            .storage_layer
            .batch_link_volunteers_to_mentors(
                project_cycle_id,
                volunteer_mentee_linkage,
                &mut exec_opts,
            )
            .await?;
    }

    // Update this job's project_cycle_id now that the project cycle exists
    services
        .storage_layer
        .set_job_project_cycle(params.job_id, project_cycle_id, &mut exec_opts)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn import_task(services: &ImportServices, params: &ImportParams) -> Result<()> {
    let data = collect_import_base_data(services, params).await?;

    match store_base_data(services, data, params).await {
        Ok(_) => {
            services
                .storage_layer
                .mark_job_complete(params.job_id, &mut ExecOptsBuilder::default().build()?)
                .await?
        }
        Err(e) => {
            services
                .storage_layer
                .mark_job_errored(
                    params.job_id,
                    e.to_string(),
                    &mut ExecOptsBuilder::default().build()?,
                )
                .await?;
        }
    };

    Ok(())
}
