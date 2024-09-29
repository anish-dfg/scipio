//! This module contains the `Authenticator` trait and two implementations: `Auth0` and `Noop`

pub mod auth0;
pub mod noop;

use anyhow::{bail, Result};
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

use super::Service;

/// User data returned by the authenticator.
///
// TODO: Ensure noop at least includes the required fields for testing purposes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum UserData {
    /// Data returned by the Auth0 authenticator.
    Auth0(auth0::UserInfo),
    /// Noop data.
    Noop,
}

/// Authentication data returned by the authenticator.
///
// TODO: Ensure noop at least includes the required fields for testing purposes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AuthData {
    /// Data returned by the Auth0 authenticator.
    Auth0(auth0::Auth0AuthData),
    /// Noop data.
    Noop,
}

impl AuthData {
    pub fn email(&self) -> Result<String> {
        match self {
            AuthData::Auth0(data) => Ok(data.email.clone()),
            AuthData::Noop => bail!("noop"),
        }
    }
}

/// The `Authenticator` trait defines the interface for authenticating users.
#[cfg_attr(test, automock)]
#[async_trait]
#[allow(unused_variables)]
pub trait Authenticator: Send + Sync {
    /// Authenticate a user using a token.
    ///
    /// * `token`: The token to authenticate with
    ///
    ///   This is typically a JWT, but the trait makes no assumptions about the format. That should
    ///   be decided by the implementation. It may be an opaque token, a JWT, or something else.
    async fn authenticate(&self, token: &str) -> Result<AuthData>;

    /// Get user info.
    ///
    /// * `token`: The token to request user info with
    async fn user_info(&self, token: &str) -> Result<UserData> {
        unimplemented!()
    }
}

pub trait AuthenticatorService: Authenticator + Service + Send + Sync {}

impl<T> AuthenticatorService for T where T: Authenticator + Service + Sync + Sync {}
