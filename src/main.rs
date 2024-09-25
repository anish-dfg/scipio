//! Scipio is the core backend service for Develop for Good's Pantheon platform.
//!
//! Pantheon is a unified platform for managing Develop for Good. It is responsible for
//! streamlining the onboarding and offboarding process for new volunteers each cycle, providing
//! analytics and insights about the organization, and hosting a variety of tools (including AI and
//! educational resources) to help volunteers succeed in their roles.
//!
//! The app is structured as a RESTful API, with a variety of endpoints for different services. The
//! `app` directory contains the actual endpoints and handlers for the API, while the `services`
//! directory contains the various clients and utilities for interacting with external services
//! (most of which are customized for and provide useful interfaces for Develop for Good's specific
//! needs).
//!
//! The services that Scipio requires to run include:
//! - An authentication provider (currently Auth0, although anything which implements
//!   `Authenticator` will work).
//! - A database (currently PostgreSQL, although anything which implements `StorageLayer` will work).
//! - An email provider (currently Sendgrid, although anything which implements `EmailClient` will
//!   suffice.
//! - A Workspace client. This is a custom service that interacts with the Google Workspace API,
//!   however the underlying implementation of this service can be swapped out, as long as it
//!   implements `WorkspaceClient`. The current implementation uses a service account and there may
//!   be a better way to handle this in the future, but that's for you to find out.
//! - An Airtable client. This is a custom service that interacts with the Airtable API. This is
//!   unlikely to change, unless Airtable dramatically changes their API. However, this service is
//!   also swappable as long as it implements `AirtableClient`.
//! - A NATS client. This is used for pub/sub messaging between services. Currently, NATS is a hard
//!   dependency and there is no underlying abstraction for this service. That's because I don't
//!   know nearly enough about pub/sub messaging to make a good abstraction. If you do, please feel
//!   free to do it yourself.
//!

// ╭───────────────────────────────────────────────────────────────────────────────────────────────────╮
// │                                             Scipio                                                │
// ╰───────────────────────────────────────────────────────────────────────────────────────────────────╯
//  @author Anish Sinha <anish@developforgood.org>
//  ────────────────────────────────────────────────────────────────────────────────────────────────────

mod app;
mod cli;
mod services;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;

use crate::app::context::ContextBuilder;
use crate::cli::Args;
use crate::services::airtable::DfgAirtableClient;
use crate::services::auth::auth0::Auth0;
use crate::services::mail::sendgrid::SendgridEmailClient;
use crate::services::storage::{Migrator, PgBackend};
#[allow(unused_imports)]
use crate::services::workspace::noop::NoopWorkspaceClient;
#[allow(unused_imports)]
use crate::services::workspace::service_account::ServiceAccountWorkspaceClient;

#[tokio::main]
async fn main() -> Result<()> {
    match dotenvy::dotenv() {
        Ok(_) => println!("loaded .env file"),
        Err(_) => println!("no .env file found"),
    };

    let args = Args::parse();

    let addr = format!("{}:{}", args.host, args.port);

    let client = async_nats::connect(&args.nats_url).await?;

    let authenticator = Arc::new(Auth0::new(&args.auth0_tenant_uri, args.auth0_audiences).await?);

    let storage_layer = Arc::new(PgBackend::new(&args.database_url).await?);
    let mail = Arc::new(SendgridEmailClient::new(&args.sendgrid_api_key, 8)?);
    let airtable = Arc::new(DfgAirtableClient::default_with_token(&args.airtable_api_token)?);
    let workspace = Arc::new(ServiceAccountWorkspaceClient::new(
        &args.workspace_client_email,
        &args.workspace_private_key_id,
        &args.workspace_private_key,
        &args.workspace_token_url,
        8,
    ));
    // let workspace = Arc::new(NoopWorkspaceClient);

    storage_layer.migrate().await?;

    let ctx = ContextBuilder::default()
        .authenticator(authenticator)
        .storage_layer(storage_layer)
        .airtable(airtable)
        .workspace(workspace)
        .nats(client)
        .mail(mail)
        .build()?;

    let srv = app::build(Arc::new(ctx)).await;

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, srv).await?;

    Ok(())
}
