//! This crate provides a simple interface to interact with the Google Workspace Admin SDK API
//! using a service account.
//!
//! @author Anish Sinha <anish@developforgood.org>

#![deny(missing_docs)]
#[cfg(test)]
mod tests;

mod retry;
pub mod user;

use anyhow::Result;
use chrono::Utc;
use derive_builder::Builder;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use retry::DefaultRetryStrategy;
use serde::{Deserialize, Serialize};
use user::{CreateWorkspaceUser, WorkspaceUser};

/// [RFC 7523 Bearer Token Grant Type](https://datatracker.ietf.org/doc/html/rfc7523#section-8.1)
const BEARER_TOKEN_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";

#[derive(Deserialize, Debug)]
/// A subset of the data from the service account JSON file. We need this to create and sign JWTs.
///
pub struct ServiceAccountJson {
    /// The email address of the service account.
    pub client_email: String,
    /// The private key ID of the service account.
    pub private_key_id: String,
    /// The private key of the service account.
    pub private_key: String,
    /// The URI of the token endpoint (where we should request a token).
    pub token_uri: String,
}

/// A structure representing the assertion that is used to get an access token from the Google
#[derive(Debug, Serialize, Deserialize, Builder)]
struct Assertion {
    /// They expected grant type for the assertion. This should always be
    /// `urn:ietf:params:oauth:grant-type:jwt-bearer` for this implementation.
    #[builder(setter(into))]
    pub grant_type: String,
    #[builder(setter(into))]
    /// The assertion itself. This is a JWT that is signed with the service account's private key.
    pub assertion: String,
}

impl TryFrom<Assertion> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: Assertion) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_string(&value)?.into_bytes())
    }
}

/// Claims for the JWT assertion used to get an access token from the Google Workspace API. More
/// details about each individual claim can be found in
/// [RFC 7523, Section 3](https://datatracker.ietf.org/doc/html/rfc7523#section-3).
#[derive(Debug, Serialize, Deserialize, Builder)]
struct AssertionClaims {
    #[builder(setter(into))]
    pub iss: String,
    #[builder(setter(into))]
    pub aud: String,
    #[builder(setter(into))]
    pub sub: String,
    #[builder(setter(into))]
    pub scope: String,
    pub iat: i64,
    pub exp: i64,
}

/// Google will return this when you request an access token with a valid assertion.
#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAccessTokenResponse {
    /// The token returned by Google. It is NOT a JWT.
    pub access_token: String,
}

/// A service account that can be used to authenticate with Google APIs.
///
/// * `json`: The JSON data of the service account.
/// * `http`: The HTTP client that will be used to make requests to the token endpoint. By default,
///   it is configured to backoff and retry on status codes 412, 429, as well as on network errors.
pub struct ServiceAccount {
    json: ServiceAccountJson,
    http: ClientWithMiddleware,
}

impl ServiceAccount {
    /// Create a new service account.
    ///
    /// * `json`: Valid service account JSON data.
    /// * `max_retries`: The maximum number of retries to attempt when making requests to the admin
    ///   directory API.
    pub fn new(json: ServiceAccountJson, max_retries: u32) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries);
        let retry_strategy = RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            DefaultRetryStrategy,
        );

        let http = ClientBuilder::new(Client::new()).with(retry_strategy).build();

        Self { json, http }
    }

    /// Request an assertion token from the Google Workspace API on the behalf of a principal.
    ///
    /// * `principal`: The email of the authenticated user requesting this action.
    /// * `scope`: The scope of the assertion token.
    fn request_assertion_token(&self, principal: &str, scope: &str) -> Result<String> {
        let claims = AssertionClaimsBuilder::default()
            .iss(&self.json.client_email)
            .aud(&self.json.token_uri)
            .sub(principal)
            .scope(scope)
            .iat(Utc::now().timestamp())
            .exp(Utc::now().timestamp() + 3600)
            .build()?;

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.json.private_key_id.clone());
        let assertion = jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(self.json.private_key.as_bytes())?,
        )?;

        Ok(assertion)
    }

    /// Get an access token from the Google Workspace API on the behalf of a principal.
    ///
    /// * `principal`: The email of the authenticated user requesting this action.
    /// * `scope`: The scope of the assertion token.
    async fn get_access_token(&self, principal: &str, scope: &str) -> Result<String> {
        let assertion_token = self.request_assertion_token(principal, scope)?;
        let assertion = AssertionBuilder::default()
            .grant_type(BEARER_TOKEN_GRANT_TYPE)
            .assertion(assertion_token)
            .build()?;

        let res = self
            .http
            .post(&self.json.token_uri)
            .body(Vec::<u8>::try_from(assertion)?)
            .send()
            .await?;
        let data = res.json::<GoogleAccessTokenResponse>().await?;

        Ok(data.access_token)
    }

    /// Create a new user in Google Workspace.
    ///
    /// * `principal`: The email of the user requesting this action.
    ///
    /// This function returns the newly created user.
    pub async fn create_user(
        &self,
        principal: &str,
        data: CreateWorkspaceUser,
    ) -> Result<WorkspaceUser> {
        let scope = "https://www.googleapis.com/auth/admin.directory.user";
        let url = "https://admin.googleapis.com/admin/directory/v1/users";
        let access_token = self.get_access_token(principal, scope).await?;

        let user = self
            .http
            .post(url)
            .bearer_auth(&access_token)
            .body(Vec::<u8>::try_from(data)?)
            .send()
            .await?
            .json::<WorkspaceUser>()
            .await?;

        Ok(user)
    }

    /// Delete a user from Google Workspace.
    ///
    /// * `principal`: The email of the user requesting this action.
    ///
    /// Delete a user from Google Workspace given their email.
    pub async fn delete_user(&self, principal: &str, email_of_user_to_delete: &str) -> Result<()> {
        let scope = "https://www.googleapis.com/auth/admin.directory.user";

        let access_token = self.get_access_token(principal, scope).await?;

        self.http
            .delete(format!(
                "https://admin.googleapis.com/admin/directory/v1/users/{email_of_user_to_delete}"
            ))
            .bearer_auth(&access_token)
            .send()
            .await?;

        Ok(())
    }
}
