pub mod entities;
mod mail_send;
mod retry;

#[cfg(test)]
mod tests;

use anyhow::Result;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use retry::DefaultRetryStrategy;

pub struct Sendgrid {
    http: ClientWithMiddleware,
}

impl Sendgrid {
    pub fn new(api_key: &str, max_retries: u32) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        let mut auth = HeaderValue::from_str(&format!("Bearer {api_key}"))?;

        auth.set_sensitive(true);
        default_headers.insert(header::AUTHORIZATION, auth);

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
        let retry_strategy = RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            DefaultRetryStrategy,
        );

        let http = ClientBuilder::new(Client::builder().default_headers(default_headers).build()?)
            .with(retry_strategy)
            .build();

        Ok(Self { http })
    }
}
