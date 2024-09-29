//! This module contains a trait for interacting with the Airtable API, as well as a concrete
//! implementation.

pub mod base_data;
pub mod entities;

#[cfg(test)]
mod tests;

use anyhow::Result;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::{Response, StatusCode};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::{
    default_on_request_failure, RetryTransientMiddleware, Retryable, RetryableStrategy,
};

use super::Service;
use crate::services::airtable::base_data::bases::AirtableBasesClient;
use crate::services::airtable::base_data::records::AirtableRecordsClient;

/// A client for interacting with the Airtable API.
///
/// * `http`: A http client parameterized with (at least) retry middleware
pub struct DfgAirtableClient {
    http: ClientWithMiddleware,
}

/// The default retry srategy for the Airtable client.
///
/// If a request fails with a 429 status code, it will be retried.
struct DefaultRetryStrategy;

impl RetryableStrategy for DefaultRetryStrategy {
    fn handle(&self, res: &Result<Response, reqwest_middleware::Error>) -> Option<Retryable> {
        match res {
            // retry if 429
            Ok(success) if success.status() == StatusCode::TOO_MANY_REQUESTS => {
                println!("Retrying request because of status code: {}", success.status());
                Some(Retryable::Transient)
            }
            // otherwise do not retry a successful request
            Ok(_) => None,
            // but maybe retry a request failure
            Err(error) => default_on_request_failure(error),
        }
    }
}

impl DfgAirtableClient {
    pub fn new(api_token: &str, max_retries: u32) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        let mut auth = HeaderValue::from_str(&format!("Bearer {api_token}"))?;

        auth.set_sensitive(true);
        default_headers.insert(header::AUTHORIZATION, auth);

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
        let retry_strategy = RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            DefaultRetryStrategy,
        );

        let http = ClientBuilder::new(
            reqwest::Client::builder().default_headers(default_headers).build()?,
        )
        .with(retry_strategy)
        .build();

        Ok(Self { http })
    }

    pub fn default_with_token(api_token: &str) -> Result<Self> {
        Self::new(api_token, 8)
    }
}

impl Service for DfgAirtableClient {
    fn get_id(&self) -> &'static str {
        "dfg-airtable-client [default]"
    }
}

/// A trait for interacting with the Airtable API.
pub trait AirtableClient: AirtableBasesClient + AirtableRecordsClient + Send + Sync {}

// Implement the AirtableClient trait for any type that implements the AirtableBasesClient and
// AirtableRecordsClient traits.
impl<T> AirtableClient for T where T: AirtableBasesClient + AirtableRecordsClient + Send + Sync {}

pub trait AirtableService: AirtableClient + Service + Send + Sync {}

impl<T> AirtableService for T where T: AirtableClient + Service + Send + Sync {}
