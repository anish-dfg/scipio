//! This module contains the implementation of the `EmailClient` trait for Sendgrid.

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::services::mail::{DefaultRetryStrategy, EmailClient, OnboardingEmailParams, TEMPLATES};

/// A client for sending emails using Sendgrid.
///
/// * `http`: A http client parameterized with (at least) retry middleware
pub struct SendgridEmailClient {
    http: ClientWithMiddleware,
}

impl SendgridEmailClient {
    /// Create a new Sendgrid email client.
    ///
    /// * `api_key`: A Sendgrid API key
    /// * `max_retries`: The maximum number of retries to attempt
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

/// The `From` struct represents the sender of an email.
///
/// * `email`: The email address of the sender
/// * `name`: The name of the sender
///
/// More information about this definition can be found
/// [here](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct From {
    email: String,
    name: String,
}

/// The `Recipient` struct represents a recipient of an email.
///
/// * `email`: The email address of the recipient
/// * `name`: The name of the recipient
///
/// More information about this definition can be found
/// [here](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Recipient {
    email: String,
    name: String,
}

/// A Sendgrid personalization object.
///
/// * `from`: The sender of the email
/// * `to`: The recipients of the email
/// * `cc`: The cc recipients of the email
/// * `bcc`: The bcc recipients of the email
///
/// More information about this definition can be found
/// [here](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send)
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub(super) struct Personalization {
    #[builder(setter(into), default = "None")]
    from: Option<From>,
    to: Vec<Recipient>,
    #[builder(setter(into), default = "None")]
    cc: Option<Vec<Recipient>>,
    #[builder(setter(into), default = "None")]
    bcc: Option<Vec<Recipient>>,
}

/// Allowed MIME types for email content.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum AllowedMimeType {
    /// Specifies plain text content
    #[serde(rename = "text/plain")]
    PlainText,
    /// Specifies HTML content
    #[serde(rename = "text/html")]
    Html,
}

/// The `Content` struct represents the content of an email.
///
/// * `mime`: The MIME type of the content
/// * `value`: The raw text or HTML content of the email as a string
///
/// More information about this definition can be found
/// [here](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Content {
    #[serde(rename = "type")]
    mime: AllowedMimeType,
    value: String,
}

/// A Sendgrid email object.
///
/// * `personalizations`: The personalizations of the email
/// * `from`: The sender of the email
/// * `subject`: The subject of the email
/// * `content`: The content of the email
/// * `send_at`: When to send the email. If `None`, the email will be sent immediately.
///   Otherwise, it will be sent at the specified UNIX timestamp (in seconds).
///
/// NOTE: This struct is not exhaustive and only includes a subset of allowed fields. More can be
/// added as needed.
///
/// More information about this definition can be found
/// [here](https://www.twilio.com/docs/sendgrid/api-reference/mail-send/mail-send)
///
// TODO: Complete this struct with all the fields needed to send an email
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub(super) struct Email {
    personalizations: Vec<Personalization>,
    from: From,
    #[builder(setter(into))]
    subject: String,
    content: Vec<Content>,
    #[builder(setter(into), default = "None")]
    send_at: Option<u64>,
}

impl TryFrom<Email> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: Email) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_string(&value)?.into_bytes())
    }
}

impl TryFrom<OnboardingEmailParams> for Email {
    type Error = anyhow::Error;

    fn try_from(params: OnboardingEmailParams) -> Result<Self> {
        let personalization = PersonalizationBuilder::default()
            .to(vec![Recipient {
                email: params.email,
                name: format!("{} {}", params.first_name, params.last_name),
            }])
            .build()?;

        let personalizations = vec![personalization];

        let from =
            From { email: "pantheon@developforgood.org".to_owned(), name: "Pantheon".to_owned() };

        let subject = "Welcome to Pantheon!".to_owned();
        let mut context = Context::new();

        context.insert("name", &params.first_name);
        context.insert("email", &params.workspace_email);
        context.insert("temporaryPassword", &params.temporary_password);

        let template = TEMPLATES.render("email/onboard.html", &context).unwrap();

        let content = vec![Content { mime: AllowedMimeType::Html, value: template }];

        let email = EmailBuilder::default()
            .personalizations(personalizations)
            .from(from)
            .subject(subject)
            .content(content)
            .send_at(params.send_at)
            .build()?;

        Ok(email)
    }
}

#[async_trait]
impl EmailClient for SendgridEmailClient {
    async fn send_onboarding_email(&self, params: OnboardingEmailParams) -> Result<()> {
        let email = Email::try_from(params)?;
        let body = Vec::try_from(email)?;

        self.http
            .post("https://api.sendgrid.com/v3/mail/send")
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?;

        Ok(())
    }
}
