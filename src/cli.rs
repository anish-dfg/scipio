//! This module defines the command line interface to Scipio.

use std::env;
use std::sync::Arc;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use scipio_airtable::Airtable;
use scipio_sendgrid::Sendgrid;
use scipio_workspace::{ServiceAccount, ServiceAccountJson};
use serde::Serialize;

use crate::app::state::{Services, ServicesBuilder};
use crate::services::airtable::AirtableService;
use crate::services::auth::auth0::Auth0;
use crate::services::auth::noop::NoopAuthenticator;
use crate::services::auth::AuthenticatorService;
use crate::services::mail::noop::NoopEmailClient;
use crate::services::mail::MailService;
use crate::services::storage::{PgBackend, StorageService};
use crate::services::workspace::noop::NoopWorkspaceClient;
use crate::services::workspace::WorkspaceService;

#[derive(ValueEnum, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum LaunchMode {
    Development,
    Production,
}

#[derive(ValueEnum, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AuthServiceImpl {
    Noop,
    Auth0,
}

#[derive(ValueEnum, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum MailServiceImpl {
    Noop,
    Sendgrid,
}

#[derive(ValueEnum, Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceServiceImpl {
    Noop,
    ServiceAccount,
}

/// Command line arguments for Pantheon
///
/// * `host`: The host to bind the server to
/// * `port`: The port to bind the server to
/// * `auth0_tenant_uri`: The Auth0 tenant URI
///
///    This should be changed to Option<String> in the future. Right now,
///    the app assumes that the authentication backend is Auth0.
///
/// * `auth0_audiences`: The Auth0 audiences
///
///    This should be changed to Option<Vec<String>> in the future. Right
///    now, the app assumes that the authentication backend is Auth0.
///
/// * `workspace_client_email`: The email of the service account to use for the Workspace API
///
///    If a better approach to interacting with Google Workspace programmatically is
///    found, replace this property and both `workspace_private_key_id` and `workspace_private_key`
///    with something else.
///
/// * `workspace_private_key_id`: The private key ID of the service account to use for the Workspace API
///
///    This is used to get an opaque access token that can be
///    used to interact with Google Workspace
///
/// * `workspace_private_key`: The private key of the service account to use for the Workspace API
/// * `workspace_token_url`: The token URL for the Workspace API
/// * `airtable_api_token`: The Airtable API token
/// * `database_url`: The URL of the database to connect to
///
/// * `sendgrid_api_key`: The Sendgrid API key
///
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env, default_value = "http://localhost")]
    pub host: String,
    #[arg(long, env, default_value = "8888")]
    pub port: String,

    #[arg(long,env,value_enum,default_value_t=LaunchMode::Production)]
    pub launch_mode: LaunchMode,

    #[arg(long, env, value_enum, default_value_t = AuthServiceImpl::Auth0)]
    pub auth_service: AuthServiceImpl,
    #[arg(long, env)]
    pub auth0_tenant_uri: Option<String>,
    #[arg(long, env, value_delimiter = ',')]
    pub auth0_audiences: Option<Vec<String>>,

    #[arg(long, env, value_enum, default_value_t = WorkspaceServiceImpl::Noop)]
    pub workspace_service: WorkspaceServiceImpl,

    #[arg(long, env)]
    pub workspace_service_account_json: String,

    #[arg(long, env)]
    pub airtable_api_token: String,

    #[arg(long, env, default_value = "postgresql://postgres:postgres@localhost:5432/postgres")]
    pub database_url: String,

    #[arg(long, env, value_enum, default_value_t = MailServiceImpl::Sendgrid)]
    pub mail_service: MailServiceImpl,
    #[arg(long, env)]
    pub sendgrid_api_key: Option<String>,
}

impl Args {
    async fn init_auth_service(&self) -> Result<Arc<dyn AuthenticatorService>> {
        let service: Arc<dyn AuthenticatorService> = match self.auth_service {
            AuthServiceImpl::Noop => Arc::new(NoopAuthenticator),
            AuthServiceImpl::Auth0 => {
                match (self.auth0_tenant_uri.as_ref(), self.auth0_audiences.as_ref()) {
                    (Some(tenant_uri), Some(audiences)) => {
                        Arc::new(Auth0::new(tenant_uri, audiences.clone()).await?)
                    }
                    _ => bail!(
                        "Auth0 tenant URI and audiences must be provided if auth service is auth0"
                    ),
                }
            }
        };
        Ok(service)
    }

    fn init_mail_service(&self) -> Result<Arc<dyn MailService>> {
        let service: Arc<dyn MailService> = match self.mail_service {
            MailServiceImpl::Noop => Arc::new(NoopEmailClient),
            MailServiceImpl::Sendgrid => match self.sendgrid_api_key.as_ref() {
                Some(api_key) => Arc::new(Sendgrid::new(api_key, 3)?),
                _ => bail!("Sendgrid API key must be provided if mail service is sendgrid"),
            },
        };
        Ok(service)
    }

    fn init_workspace_service(&self) -> Result<Arc<dyn WorkspaceService>> {
        let service_account_json = env::var("WORKSPACE_SERVICE_ACCOUNT_JSON")?;
        let data = serde_json::from_str::<ServiceAccountJson>(&service_account_json)?;

        let service: Arc<dyn WorkspaceService> = match self.workspace_service {
            WorkspaceServiceImpl::Noop => Arc::new(NoopWorkspaceClient),
            WorkspaceServiceImpl::ServiceAccount => Arc::new(ServiceAccount::new(data, 5)),
        };

        Ok(service)
    }

    fn init_airtable_service(&self) -> Result<Arc<dyn AirtableService>> {
        Ok(Arc::new(Airtable::new(&self.airtable_api_token, 5)?))
    }

    async fn init_storage_service(&self) -> Result<Arc<dyn StorageService>> {
        Ok(Arc::new(PgBackend::new(&self.database_url).await?))
    }

    pub async fn init_services(&self) -> Result<Arc<Services>> {
        Ok(Arc::new(
            ServicesBuilder::default()
                .authenticator(self.init_auth_service().await?)
                .storage_layer(self.init_storage_service().await?)
                .airtable(self.init_airtable_service()?)
                .workspace(self.init_workspace_service()?)
                .mail(self.init_mail_service()?)
                .build()?,
        ))
    }
}
