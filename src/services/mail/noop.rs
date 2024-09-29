use anyhow::Result;
use async_trait::async_trait;

use super::{EmailClient, OnboardingEmailParams};
use crate::services::Service;

pub struct NoopEmailClient;

#[async_trait]
impl EmailClient for NoopEmailClient {
    async fn send_onboarding_email(&self, _params: OnboardingEmailParams) -> Result<()> {
        Ok(())
    }
}

impl Service for NoopEmailClient {
    fn get_id(&self) -> &'static str {
        "noop"
    }
}
