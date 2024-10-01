//! Responses returned from the Airtable Base API

use serde::{Deserialize, Serialize};

use crate::services::airtable::entities::bases::Base;
use crate::services::airtable::entities::schema::Table;

/// Base response from the Airtable API.
///
/// * `offset`: The offset to start listing bases from if we need to fetch more.
/// * `bases`: The bases returned from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBasesResponse {
    pub offset: Option<String>,
    pub bases: Vec<Base>,
}

/// Schema response from the Airtable API.
///
/// * `tables`: The tables returned from the API that are associated with the base.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaResponse {
    pub tables: Vec<Table>,
}

/// A trait to validate the schema of a base.
pub trait V1SchemaValidator {
    fn validate(&self) -> bool;
}

// For volunteers + mentors
const REQUIRED_VOLUNTEER_TABLE_FIELDS: [&str; 19] = [
    "FirstName",
    "LastName",
    "Email",
    "Phone",
    "Gender",
    "Ethnicity",
    "AgeRange",
    "OrgName (from ProjectRecordID)",
    "Company",
    "JobTitle",
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

const REQUIRED_VOLUNTEER_VIEWS: [&str; 4] = [
    "All Committed Student Volunteers - Active",
    "All Committed Student + Management Volunteers",
    "All Committed Mentor Volunteers",
    "All Committed Mentor Volunteers - 1:1 Mentor-Mentee Pairings",
];

// For nonprofits
const REQUIRED_NONPROFIT_TABLE_FIELDS: [&str; 17] = [
    "FirstName",
    "LastName",
    "JobTitle",
    "NonprofitEmail",
    "Cc",
    "OrgName",
    "ProjectName",
    "Agreement and Invoice Sent",
    "ServicesAgreementSignature",
    "AvailabilityConfirmed",
    "InvoicePaid",
    "OrgWebsite",
    "CountryHQ",
    "StateHQ",
    "Address",
    "Size",
    "ImpactCauses",
];

const REQUIRED_NONPROFIT_VIEWS: [&str; 1] = [
    "Accepted Nonprofit Projects",
    // r"Finalized\s.*Nonprofit Projects",
    // "Finalized Sum24 Nonprofit Projects",
];

impl V1SchemaValidator for SchemaResponse {
    fn validate(&self) -> bool {
        let tables = &self.tables;
        let volunteers_table = tables.iter().find(|t| t.name == "Volunteers");
        let nonprofits_table = tables.iter().find(|t| t.name == "Nonprofits");
        match (volunteers_table, nonprofits_table) {
            (Some(v), Some(n)) => {
                let v_has_required_fields =
                    REQUIRED_VOLUNTEER_TABLE_FIELDS.iter().all(|required_field| {
                        v.fields
                            .iter()
                            .map(|f| f.name.as_ref())
                            .collect::<Vec<_>>()
                            .contains(required_field)
                    });

                let v_has_required_views = REQUIRED_VOLUNTEER_VIEWS.iter().all(|required_view| {
                    v.views
                        .iter()
                        .map(|v| v.name.as_ref())
                        .collect::<Vec<_>>()
                        .contains(required_view)
                });

                let n_has_required_dynamic_view = n.views.iter().any(|v| {
                    v.name.starts_with(&"Finalized".to_owned())
                        && v.name.ends_with(&"Nonprofit Projects".to_owned())
                });

                let n_has_required_fields =
                    REQUIRED_NONPROFIT_TABLE_FIELDS.iter().all(|required_field| {
                        n.fields
                            .iter()
                            .map(|f| f.name.as_ref())
                            .collect::<Vec<_>>()
                            .contains(required_field)
                    });

                let n_has_required_views = REQUIRED_NONPROFIT_VIEWS.iter().all(|required_view| {
                    n.views
                        .iter()
                        .map(|v| v.name.as_ref())
                        .collect::<Vec<_>>()
                        .contains(required_view)
                });

                v_has_required_fields
                    && n_has_required_fields
                    && v_has_required_views
                    && n_has_required_views
                    && n_has_required_dynamic_view
            }
            (_, _) => false,
        }
    }
}
