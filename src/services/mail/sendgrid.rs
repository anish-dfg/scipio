use anyhow::Result;
use async_trait::async_trait;
use scipio_sendgrid::entities::Mail;
use scipio_sendgrid::Sendgrid;

use super::{EmailClient, OnboardingEmailParams};
use crate::services::Service;

#[async_trait]
impl EmailClient for Sendgrid {
    async fn send_onboarding_email(&self, params: OnboardingEmailParams) -> Result<()> {
        let mail = Mail::try_from(params)?;
        self.send_mail(mail).await?;
        Ok(())
    }
}

impl Service for Sendgrid {
    fn get_id(&self) -> &'static str {
        "sendgrid"
    }
}
