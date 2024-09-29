use anyhow::Result;
use async_trait::async_trait;

use super::Authenticator;
use crate::services::auth::AuthData;
use crate::services::Service;

/// A Noop authenticator that always returns `AuthData::Noop`.
pub struct NoopAuthenticator;

#[async_trait]
impl Authenticator for NoopAuthenticator {
    async fn authenticate(&self, _: &str) -> Result<AuthData> {
        Ok(AuthData::Noop)
    }
}

impl Service for NoopAuthenticator {
    fn get_id(&self) -> &'static str {
        "noop"
    }
}
