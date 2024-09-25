use std::collections::HashMap;
use std::sync::Arc;

use async_nats::client::Client;
use derive_builder::Builder;
use sqlx::{Database, Postgres};
use tokio::sync::watch::Sender;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::services::airtable::AirtableClient;
use crate::services::auth::Authenticator;
use crate::services::mail::EmailClient;
use crate::services::storage::StorageLayer;
use crate::services::workspace::WorkspaceClient;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Context<DB: Database = Postgres> {
    pub authenticator: Arc<dyn Authenticator>,
    pub storage_layer: Arc<dyn StorageLayer<DB>>,
    pub airtable: Arc<dyn AirtableClient>,
    pub workspace: Arc<dyn WorkspaceClient>,
    pub nats: Client,
    pub mail: Arc<dyn EmailClient>,
}
