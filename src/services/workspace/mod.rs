//! This module provides interfaces and implementations for interacting with the Google Workspace
//! API. Currently, the only implementation is a service account-based implementation.

pub mod entities;
pub mod noop;
pub mod service_account;

use anyhow::Result;
use async_trait::async_trait;
use entities::CreateWorkspaceVolunteer;

use super::Service;

/// A trait for interacting with the Google Workspace API.
///
/// Any type which implements this trait may
/// be injected as a dependency in the application. The functions in this trait take a `principal`.
/// This is unique to the service account implementation, and should be replaced with an enum in
/// the future as this abstraction is currently leaky.
#[async_trait]
#[allow(unused_variables)]
pub trait WorkspaceClient: Send + Sync {
    /// Create a new user in Google Workspace.
    ///
    /// * `principal`: The email of the user requesting this action.
    ///
    /// This function should ONLY be called with the email of the authenticated user requesting
    /// this action. The email of the authenticated user is always in the JWT in the request
    /// header. This is a security measure and we are delegating authentication to Auth0. Never
    /// call this function with user provided input. This is one reason why we should try to find
    /// an alternative to the service account approach.
    async fn create_volunteer(
        &self,
        principal: &str,
        volunteer: CreateWorkspaceVolunteer,
    ) -> Result<()> {
        unimplemented!()
    }

    /// Delete a user from Google Workspace.
    ///
    /// * `principal`: The email of the user requesting this action.
    ///
    /// This function should ONLY be called with the email of the authenticated user requesting
    /// this action. The email of the authenticated user is always in the JWT in the request
    /// header. This is a security measure and we are delegating authentication to Auth0. Never
    /// call this function with user provided input. This is one reason why we should try to find
    /// an alternative to the service account approach.
    #[allow(unused)]
    async fn delete_user(&self, principal: &str, email_of_user_to_delete: &str) -> Result<()> {
        unimplemented!()
    }
}

pub trait WorkspaceService: WorkspaceClient + Service + Send + Sync {}
impl<T> WorkspaceService for T where T: WorkspaceClient + Service + Send + Sync {}
