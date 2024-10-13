//! This module contains a client for interacting with the Google Workspace API using a service
//!
//!

use anyhow::Result;
use async_trait::async_trait;
use scipio_workspace::user::CreateWorkspaceUser;
use scipio_workspace::ServiceAccount;

use super::entities::CreateWorkspaceVolunteer;
use super::WorkspaceClient;
use crate::services::Service;

#[async_trait]
impl WorkspaceClient for ServiceAccount {
    async fn create_volunteer(
        &self,
        principal: &str,
        volunteer: CreateWorkspaceVolunteer,
    ) -> Result<()> {
        let user = CreateWorkspaceUser::try_from(volunteer)?;

        let _ = self.create_user(principal, user).await?;

        Ok(())
    }

    async fn delete_user(&self, principal: &str, email_of_user_to_delete: &str) -> Result<()> {
        self.delete_user(principal, email_of_user_to_delete).await?;
        Ok(())
    }
}

impl Service for ServiceAccount {
    fn get_id(&self) -> &'static str {
        "scipio-service-account"
    }
}
