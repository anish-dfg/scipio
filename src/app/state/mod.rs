use std::sync::Arc;

use derive_builder::Builder;
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

impl Services {
    pub fn get_info(&self) -> String {
        format!(
            "Configured Services: {{
    authenticator: {},
    airtable: {},
    storage: {},
    workspace: {},
    mail: {}
}}",
            self.authenticator.get_id(),
            self.airtable.get_id(),
            self.storage_layer.get_id(),
            self.workspace.get_id(),
            self.mail.get_id(),
        )
    }
}
