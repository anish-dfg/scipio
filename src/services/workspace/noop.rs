//! This module defines a no-op implementation of the `WorkspaceClient` trait.

use anyhow::Result;
use axum::async_trait;

use crate::services::workspace::entities::CreateWorkspaceUser;
use crate::services::workspace::WorkspaceClient;

/// A no-op implementation of the `WorkspaceClient` trait.
///
/// This implementation does nothing, simply
/// returning `Ok(())` for all operations. This is useful for testing and normal development, as most
/// of the time you don't want to actually perform workspace operations. Google Workspace
/// operations are costly (in terms of time, not money), and are annoying to undo. Google takes
/// forever to update after changes, so it's not worth using an actual implementation in
/// development and testing. That's also why all the tests in this module are marked with `#[ignore]`
/// so you don't run them unless you're explicit about it.
pub struct NoopWorkspaceClient;

#[async_trait]
impl WorkspaceClient for NoopWorkspaceClient {
    async fn create_user(&self, _principal: &str, _user: CreateWorkspaceUser) -> Result<()> {
        Ok(())
    }

    async fn delete_user(&self, _principal: &str, _email_of_user_to_delete: &str) -> Result<()> {
        Ok(())
    }
}
