use std::collections::HashMap;

use anyhow::Result;
use uuid::Uuid;

use super::ImportServices;
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
}

#[derive(Debug)]
struct ImportBaseData {
    name: String,
    description: String,
    nonprofits: Vec<CreateNonprofit>,
    volunteers: Vec<CreateVolunteer>,
    volunteer_nonprofit_linkage: Vec<(String, String)>,
    mentors: Vec<CreateMentor>,
    mentor_nonprofit_linkage: Vec<(String, String)>,
    mentor_mentee_linkage: Vec<(String, String)>,
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
        .filter_map(|(email, org_name)| {
            let mentor_id = mentor_email_id_map.get(email)?;
            let nonprofit_id = nonprofit_name_id_map.get(org_name)?;
            Some((*mentor_id, *nonprofit_id))
        })
        .collect::<Vec<(Uuid, Uuid)>>();

    let volunteer_mentee_linkage = data
        .mentor_mentee_linkage
        .iter()
        .filter_map(|(mentor_email, mentee_email)| {
            let mentor_id = mentor_email_id_map.get(mentor_email);
            let mentee_id = volunteer_email_id_map.get(mentee_email);
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

    services
        .storage_layer
        .set_job_project_cycle(params.job_id, project_cycle_id, &mut exec_opts)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn import_task(services: &ImportServices, params: &ImportParams) -> Result<()> {
    let volunteer_records = services.airtable.list_volunteers(&params.base_id).await?;

    let volunteer_nonprofit_linkage = volunteer_records
        .iter()
        .flat_map(|volunteer| {
            volunteer.org_name.iter().map(|org_name| (volunteer.email.clone(), org_name.clone()))
        })
        .collect::<Vec<_>>();

    let volunteers = volunteer_records.into_iter().map(CreateVolunteer::from).collect::<Vec<_>>();

    let mentor_records = services.airtable.list_mentors(&params.base_id).await?;

    let mentor_nonprofit_linkage = mentor_records
        .iter()
        .filter_map(|mentor| {
            if mentor.project_roles.contains(&"Team Mentor".to_owned()) {
                Some(
                    mentor.org_name.iter().map(|org_name| (mentor.email.clone(), org_name.clone())),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let mentors = mentor_records.into_iter().map(CreateMentor::from).collect::<Vec<_>>();

    let nonprofits = services
        .airtable
        .list_nonprofits(&params.base_id)
        .await?
        .into_iter()
        .map(CreateNonprofit::from)
        .collect::<Vec<_>>();

    let mentor_mentee_linkage = services
        .airtable
        .get_mentor_mentee_linkages(&params.base_id)
        .await?
        .iter()
        .flat_map(|linkage| {
            linkage
                .mentee_email
                .iter()
                .map(|mentee_email| (linkage.mentor_email.clone(), mentee_email.clone()))
        })
        .collect::<Vec<_>>();

    let data = ImportBaseData {
        name: params.name.clone(),
        description: params.description.clone(),
        nonprofits,
        volunteers,
        volunteer_nonprofit_linkage,
        mentors,
        mentor_mentee_linkage,
        mentor_nonprofit_linkage,
    };

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
