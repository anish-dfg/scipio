//! This module contains traits for sending emails, as well as one concrete implementation (SendGrid).

pub mod sendgrid;
#[cfg(test)]
mod tests;

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use lazy_static::lazy_static;
use reqwest::{Response, StatusCode};
use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};
use tera::Tera;

lazy_static! {
    /// The Tera instance for rendering email templates.
    static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

/// The default retry srategy for the email client.
///
/// Currently, this strategy retries requests that fail with a 429 status code.
struct DefaultRetryStrategy;

impl RetryableStrategy for DefaultRetryStrategy {
    fn handle(&self, res: &Result<Response, reqwest_middleware::Error>) -> Option<Retryable> {
        match res {
            // retry if 429
            Ok(success) if success.status() == StatusCode::TOO_MANY_REQUESTS => {
                println!("Retrying request because of status code: {}", success.status());
                dbg!(success.status());
                log::info!("Retrying request because of status code: {}", success.status());
                Some(Retryable::Transient)
            }
            // otherwise do not retry a successful request
            Ok(_) => None,
            // but maybe retry a request failure
            Err(error) => default_on_request_failure(error),
        }
    }
}

/// Data needed to send an onboarding email.
///
/// * `first_name`: The recipient's first name
/// * `last_name`: The recipient's last name
/// * `email`: The recipient's email address
/// * `workspace_email`: The recipient's workspace email address
/// * `temporary_password`: The recipient's temporary password for their workspace email address
/// * `send_at`: The time to send the email. If `None`, the email will be sent immediately.
///   Otherwise, it will be interpreted as a UNIX timestamp in seconds.
#[derive(Debug, Clone, Builder)]
pub struct OnboardingEmailParams {
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub email: String,
    #[builder(setter(into))]
    pub workspace_email: String,
    #[builder(setter(into))]
    pub temporary_password: String,
    #[builder(setter(into), default = "None")]
    pub send_at: Option<u64>,
}

#[async_trait]
pub trait EmailClient: Send + Sync {
    /// Sends an onboarding email.
    ///
    /// * `params`: Data needed to send the onboarding email
    async fn send_onboarding_email(&self, params: OnboardingEmailParams) -> Result<()>;
}
