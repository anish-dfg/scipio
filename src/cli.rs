//! This module defines the command line interface to Pantheon.

use clap::Parser;

#[derive(Parser, Debug)]
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
///    Currently, the only database impolentation is
///    Postgres, but this could, in theory, be swapped out for another database implementation,
///    though that would require a lot of work for no gain.
///
/// * `cache_url`: The URL of the cache to connect to
///
///    Currently, this parameter is ignored because we haven't
///    actually implemented a caching backend.
///
/// * `sendgrid_api_key`: The Sendgrid API key
///
/// * `nats_url`: The URL of the NATS server to connect to
pub struct Args {
    #[arg(long, env)]
    pub host: String,
    #[arg(long, env)]
    pub port: String,

    #[arg(long, env)]
    pub auth0_tenant_uri: String,
    #[arg(long, env, value_delimiter = ',')]
    pub auth0_audiences: Vec<String>,

    #[arg(long, env)]
    pub workspace_client_email: String,
    #[arg(long, env)]
    pub workspace_private_key_id: String,
    #[arg(long, env)]
    pub workspace_private_key: String,
    #[arg(long, env, default_value = "https://oauth2.googleapis.com/token")]
    pub workspace_token_url: String,

    #[arg(long, env)]
    pub airtable_api_token: String,

    #[arg(long, env, default_value = "postgresql://postgres:postgres@localhost:5432/postgres")]
    pub database_url: String,

    #[arg(long, env, default_value = "redis://redis@localhost")]
    pub cache_url: String,

    #[arg(long, env)]
    pub sendgrid_api_key: String,

    #[arg(long, env)]
    pub nats_url: String,
}
