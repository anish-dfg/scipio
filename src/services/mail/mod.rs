//! This module contains traits for sending emails, as well as one concrete implementation (SendGrid).

pub mod noop;
pub mod sendgrid;
#[cfg(test)]
mod tests;

use std::env;

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use lazy_static::lazy_static;
use scipio_sendgrid::entities::{
    AddressBuilder, Mail, MailBuilder, MailContentBuilder, MailContentMime, PersonalizationBuilder,
};
use tera::{Context, Tera};

use super::Service;

lazy_static! {

    /// The Tera instance for rendering email templates.
    static ref TEMPLATES: Tera = {
        let templates_dir = env::var("MAIL_TEMPLATES_DIR").unwrap_or_else(|_| "templates".to_owned());
        // log::info!("Loading templates from {}", templates_dir);
        match Tera::new(&format!("{templates_dir}/**/*")) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
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

impl TryFrom<OnboardingEmailParams> for Mail {
    type Error = anyhow::Error;

    fn try_from(value: OnboardingEmailParams) -> std::result::Result<Self, Self::Error> {
        let personalization = PersonalizationBuilder::default()
            .to(vec![AddressBuilder::default()
                .email(value.email)
                .name(format!("{} {}", value.first_name, value.last_name))
                .build()?])
            .build()?;

        let from = AddressBuilder::default()
            .email("onboarding@developforgood.org")
            .name("Develop for Good".to_owned())
            .build()?;

        let subject = "Develop for Good: Onboarding instructions".to_owned();
        let mut context = Context::new();

        context.insert("name", &value.first_name);
        context.insert("email", &value.workspace_email);
        context.insert("temporaryPassword", &value.temporary_password);

        let template = TEMPLATES.render("email/onboard.html", &context)?;

        let content = MailContentBuilder::default()
            .value(template)
            .mime_type(MailContentMime::Html)
            .build()?;

        let mail = MailBuilder::default()
            .from(from)
            .personalizations(vec![personalization])
            .subject(subject)
            .content(vec![content])
            .send_at(value.send_at)
            .build()?;

        Ok(mail)
    }
}

#[async_trait]
pub trait EmailClient: Send + Sync {
    /// Sends an onboarding email.
    ///
    /// * `params`: Data needed to send the onboarding email
    async fn send_onboarding_email(&self, params: OnboardingEmailParams) -> Result<()>;
}

pub trait MailService: EmailClient + Service + Send + Sync {}

impl<T> MailService for T where T: EmailClient + Service + Send + Sync {}
