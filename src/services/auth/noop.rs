use anyhow::Result;
use async_trait::async_trait;

use super::Authenticator;
use crate::services::auth::AuthData;

/// A Noop authenticator that always returns `AuthData::Noop`.
pub struct Noop;

#[async_trait]
impl Authenticator for Noop {
    async fn authenticate(&self, _: &str) -> Result<AuthData> {
        Ok(AuthData::Noop)
    }
}
