use std::sync::Arc;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Database, Postgres};

use crate::services::airtable::AirtableService;
use crate::services::auth::AuthenticatorService;
use crate::services::mail::MailService;
use crate::services::storage::StorageService;
use crate::services::workspace::WorkspaceService;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Services<DB: Database = Postgres> {
    pub authenticator: Arc<dyn AuthenticatorService>,
    pub storage_layer: Arc<dyn StorageService<DB>>,
    pub airtable: Arc<dyn AirtableService>,
    pub workspace: Arc<dyn WorkspaceService>,
    pub mail: Arc<dyn MailService>,
}

// pub struct ServiceInfo {
//     id: String,
// }

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfiguredServices<'a> {
    pub authenticator: &'a str,
    pub airtable: &'a str,
    pub storage: &'a str,
    pub workspace: &'a str,
    pub mail: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiServiceDetails<'a> {
    pub configured_services: ConfiguredServices<'a>,
}

impl Services {
    pub fn get_info(&self) -> ApiServiceDetails {
        ApiServiceDetails {
            configured_services: ConfiguredServices {
                authenticator: self.authenticator.get_id(),
                airtable: self.airtable.get_id(),
                storage: self.storage_layer.get_id(),
                workspace: self.workspace.get_id(),
                mail: self.mail.get_id(),
            },
        }
    }
}
