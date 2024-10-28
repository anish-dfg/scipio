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
//!

// ╭───────────────────────────────────────────────────────────────────────────────────────────────────╮
// │                                             Scipio                                                │
// ╰───────────────────────────────────────────────────────────────────────────────────────────────────╯
//  @author Anish Sinha <anish@developforgood.org>
//  ────────────────────────────────────────────────────────────────────────────────────────────────────

#![forbid(unsafe_code)]

mod app;
mod cli;
mod services;

use std::env;

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;

use crate::cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    match dotenvy::dotenv() {
        Ok(_) => log::info!("loaded .env file"),
        Err(_) => log::info!("no .env file found"),
    };

    let templates_dir = env::var("MAIL_TEMPLATES_DIR").unwrap_or_else(|_| "templates".to_owned());
     log::info!("Loading templates from {}", templates_dir);

    let args = Args::parse();
    log::info!(
        "MAIL RECIPIENT OVERRIDE: {}",
        env::var("MAIL_RECIPIENT_OVERRIDE").unwrap_or_default()
    );

    let addr = format!("{}:{}", args.host, args.port);

    let services = args.init_services().await?;

    services.storage_layer.migrate().await?;

    log::info!("successfully ran database migrations");

    log::info!("{:?}", services.get_info());

    let srv = app::build(services).await;

    let listener = TcpListener::bind(&addr).await?;

    log::info!("started scipio server on {}", addr);

    axum::serve(listener, srv).await?;

    Ok(())
}
