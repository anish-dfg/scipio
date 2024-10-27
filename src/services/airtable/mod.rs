pub mod entities;

#[cfg(test)]
mod tests;

use anyhow::{bail, Result};
use async_trait::async_trait;
use entities::{Mentor, MentorMenteeLinkage, Nonprofit, Volunteer};
use scipio_airtable::base_data::entities::Base;
use scipio_airtable::base_data::records::ListRecordsQueryBuilder;
use scipio_airtable::Airtable;

use super::Service;

const VOLUNTEERS_VIEW: &str = "All Committed Student Volunteers - Active";
const MENTORS_VIEW: &str = "All Committed Mentor Volunteers";
const NONPROFITS_VIEW_PREFIX: &str = "Finalized";
const NONPROFITS_VIEW_SUFFIX: &str = "Nonprofit Projects";
const MENTOR_MENTEE_LINKAGE_VIEW: &str =
    "All Committed Mentor Volunteers - 1:1 Mentor-Mentee Pairings";

const REQUIRED_VOLUNTEER_FIELDS: [&str; 17] = [
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
];

const REQUIRED_MENTOR_FIELDS: [&str; 16] = [
    "FirstName",
    "LastName",
    "Email",
    "Phone",
    "Company",
    "JobTitle",
    "OrgName (from ProjectRecordID)",
    "Country",
    "State",
    "YearsExperience",
    "ExperienceLevel",
    "PriorMentorship",
    "PriorDFG",
    "University",
    "HearAbout",
    "ProjectRole",
];

const REQUIRED_NONPROFIT_FIELDS: [&str; 13] = [
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
];

const REQUIRED_MENTOR_MENTEE_LINKAGE_FIELDS: [&str; 2] =
    ["Email", "Mentee Email (from Volunteers)"];

#[async_trait]
#[allow(unused_variables)]
pub trait AirtableClient: Send + Sync {
    async fn list_available_bases(&self) -> Result<Vec<Base>> {
        unimplemented!()
    }

    async fn list_volunteers(&self, base_id: &str) -> Result<Vec<Volunteer>> {
        unimplemented!()
    }

    async fn list_mentors(&self, base_id: &str) -> Result<Vec<Mentor>> {
        unimplemented!()
    }

    async fn list_nonprofits(&self, base_id: &str) -> Result<Vec<Nonprofit>> {
        unimplemented!()
    }

    async fn get_mentor_mentee_linkages(&self, base_id: &str) -> Result<Vec<MentorMenteeLinkage>> {
        unimplemented!()
    }

    async fn validate_schema(&self, base_id: &str) -> Result<bool> {
        unimplemented!()
    }
}

#[async_trait]
impl AirtableClient for Airtable {
    async fn validate_schema(&self, base_id: &str) -> Result<bool> {
        let schema = self.get_base_schema(base_id, vec![]).await?;

        let (Some(volunteers_table), Some(_)) = (
            schema.tables.iter().find(|table| table.name == "Volunteers"),
            schema.tables.iter().find(|table| table.name == "Nonprofits"),
        ) else {
            return Ok(false);
        };

        let v_has_required_fields = REQUIRED_VOLUNTEER_FIELDS.iter().all(|required_field| {
            volunteers_table
                .fields
                .iter()
                .map(|f| f.name.as_ref())
                .collect::<Vec<_>>()
                .contains(required_field)
        });

        if !v_has_required_fields {
            return Ok(false);
        }

        let (Some(_), Some(_), Some(_)) = (
            volunteers_table.views.iter().find(|view| view.name == VOLUNTEERS_VIEW),
            volunteers_table.views.iter().find(|view| view.name == MENTORS_VIEW),
            volunteers_table.views.iter().find(|view| view.name == MENTOR_MENTEE_LINKAGE_VIEW),
        ) else {
            return Ok(false);
        };

        Ok(true)
    }

    async fn list_available_bases(&self) -> Result<Vec<Base>> {
        let mut offset = Option::<String>::None;

        let mut bases = Vec::<Base>::with_capacity(10);

        while let Ok(mut res) = self.list_bases(offset).await {
            bases.append(&mut res.bases);

            match res.offset {
                Some(next_offset) => offset = Some(next_offset),
                None => break,
            }
        }

        Ok(bases)
    }

    async fn list_volunteers(&self, base_id: &str) -> Result<Vec<Volunteer>> {
        let mut query = ListRecordsQueryBuilder::default()
            .view(VOLUNTEERS_VIEW.to_owned())
            .fields(REQUIRED_VOLUNTEER_FIELDS.map(ToString::to_string).to_vec())
            .build()?;

        let mut volunteers = Vec::<Volunteer>::with_capacity(300);

        loop {
            let res = self.list_records::<Volunteer>(base_id, "Volunteers", Some(&query)).await?;

            volunteers
                .append(&mut res.records.into_iter().map(|data| data.fields).collect::<Vec<_>>());

            match res.offset {
                Some(next_offset) => query.offset = Some(next_offset),
                None => break,
            }
        }

        Ok(volunteers)
    }

    async fn list_mentors(&self, base_id: &str) -> Result<Vec<Mentor>> {
        let mut query = ListRecordsQueryBuilder::default()
            .view(MENTORS_VIEW.to_owned())
            .fields(REQUIRED_MENTOR_FIELDS.map(ToString::to_string).to_vec())
            .build()?;

        let mut mentors = Vec::<Mentor>::with_capacity(100);

        loop {
            let res = self.list_records::<Mentor>(base_id, "Volunteers", Some(&query)).await?;

            mentors
                .append(&mut res.records.into_iter().map(|data| data.fields).collect::<Vec<_>>());

            match res.offset {
                Some(next_offset) => query.offset = Some(next_offset),
                None => break,
            }
        }

        Ok(mentors)
    }

    async fn list_nonprofits(&self, base_id: &str) -> Result<Vec<Nonprofit>> {
        let schema = self.get_base_schema(base_id, vec![]).await?;
        let Some(nonprofits_table) = schema.tables.iter().find(|table| table.name == "Nonprofits")
        else {
            bail!("Nonprofits table not found in base schema")
        };

        let Some(finalized_nonprofits_view) = nonprofits_table.views.iter().find(|view| {
            view.name.starts_with(NONPROFITS_VIEW_PREFIX)
                && view.name.ends_with(NONPROFITS_VIEW_SUFFIX)
        }) else {
            bail!("Finalized nonprofits view not found in Nonprofits table")
        };

        let mut query = ListRecordsQueryBuilder::default()
            .view(finalized_nonprofits_view.id.to_owned())
            .fields(REQUIRED_NONPROFIT_FIELDS.map(ToString::to_string).to_vec())
            .build()?;

        let mut nonprofits = Vec::<Nonprofit>::with_capacity(100);

        loop {
            let res = self.list_records::<Nonprofit>(base_id, "Nonprofits", Some(&query)).await?;

            nonprofits
                .append(&mut res.records.into_iter().map(|data| data.fields).collect::<Vec<_>>());

            match res.offset {
                Some(next_offset) => query.offset = Some(next_offset),
                None => break,
            }
        }

        Ok(nonprofits)
    }

    async fn get_mentor_mentee_linkages(&self, base_id: &str) -> Result<Vec<MentorMenteeLinkage>> {
        let mut query = ListRecordsQueryBuilder::default()
            .view(MENTOR_MENTEE_LINKAGE_VIEW.to_owned())
            .fields(REQUIRED_MENTOR_MENTEE_LINKAGE_FIELDS.map(ToString::to_string).to_vec())
            .build()?;

        let mut linkages = Vec::<MentorMenteeLinkage>::with_capacity(100);
        loop {
            let res = self
                .list_records::<MentorMenteeLinkage>(base_id, "Volunteers", Some(&query))
                .await?;

            linkages
                .append(&mut res.records.into_iter().map(|data| data.fields).collect::<Vec<_>>());

            match res.offset {
                Some(next_offset) => query.offset = Some(next_offset),
                None => break,
            }
        }

        log::info!("Retrieved mentor-mentee linkages from Airtable");

        Ok(linkages)
    }
}

impl Service for Airtable {
    fn get_id(&self) -> &'static str {
        "scipio-airtable"
    }
}

pub trait AirtableService: AirtableClient + Send + Sync + Service {}

impl<T: AirtableClient + Send + Sync + Service> AirtableService for T {}
