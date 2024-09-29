//! This module contains a client for interacting with the Google Workspace API using a service
//! account.

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde::{Deserialize, Serialize};

use crate::services::workspace::entities::CreateWorkspaceUser;
use crate::services::workspace::{DefaultRetryStrategy, WorkspaceClient};
use crate::services::Service;

// #[derive(Debug, Deserialize)]
// pub struct ServiceAccount {
//     #[serde(rename = "type")]
//     _type: String,
//     project_id: String,
//     private_key_id: String,
//     private_key: String,
//     client_email: String,
//     client_id: String,
//     auth_uri: String,
//     auth_provider_x509_cert_url: String,
//     client_x509_cert_url: String,
//     universe_domain: String,
// }

/// [RFC 7523 Bearer Token Grant Type](https://datatracker.ietf.org/doc/html/rfc7523#section-8.1)
const BEARER_TOKEN_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:jwt-bearer";

/// A client for interacting with the Google Workspace API using a service account.
///
/// All fields of this structure (except http) are derived from the service account JSON file that
/// is provided by Google when you request a service account key.
///
/// This client will retry requests with an exponential backoff strategy, and will retry on 412 and
/// 429 status codes. You may swap out this strategy, but it is recommended to keep retrying on 412
/// because this often comes up when Workspace is not up to date with your changes, for example, if
/// you create and immediately try to delete a user.
pub struct ServiceAccountWorkspaceClient {
    /// The email of the service account.
    pub client_email: String,
    /// The private key ID of the service account.
    pub private_key_id: String,
    /// The private key of the service account.
    pub private_key: String,
    /// The token URI for the Workspace API.
    pub token_uri: String,
    /// The HTTP client, parameterized with middleware, to use for requests.
    pub http: ClientWithMiddleware,
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
    pub access_token: String,
}

impl ServiceAccountWorkspaceClient {
    /// Create a new `ServiceAccountWorkspaceClient`.
    ///
    /// * `client_email`: The email of the service account.
    /// * `private_key_id`: The private key ID of the service account.
    /// * `private_key`: The private key of the service account.
    /// * `token_uri`: The token URI for the service account.
    /// * `max_retries`: The maximum number of retries to attempt for a request. This is used for
    ///   the exponential backoff retry strategy.
    pub fn new(
        client_email: &str,
        private_key_id: &str,
        private_key: &str,
        token_uri: &str,
        max_retries: u64,
    ) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries as u32);
        let retry_strategy = RetryTransientMiddleware::new_with_policy_and_strategy(
            retry_policy,
            DefaultRetryStrategy,
        );

        let http = ClientBuilder::new(Client::new()).with(retry_strategy).build();

        Self {
            client_email: client_email.into(),
            private_key_id: private_key_id.into(),
            private_key: private_key.into(),
            token_uri: token_uri.into(),
            http,
        }
    }

    // pub fn new_from_service_account_json(
    //     service_account: ServiceAccount,
    //     max_retries: u64,
    // ) -> Self {
    //     let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_retries as u32);
    //     let retry_strategy = RetryTransientMiddleware::new_with_policy_and_strategy(
    //         retry_policy,
    //         DefaultRetryStrategy,
    //     );
    //
    //     let http = ClientBuilder::new(Client::new()).with(retry_strategy).build();
    //
    //     Self {
    //         client_email: service_account.client_email,
    //         private_key_id: service_account.private_key_id,
    //         private_key: service_account.private_key,
    //         token_uri: service_account.auth_provider_x509_cert_url,
    //         http,
    //     }
    // }

    /// Request an assertion token from the Google Workspace API on the behalf of a principal.
    ///
    /// * `principal`: The email of the authenticated user requesting this action.
    /// * `scope`: The scope of the assertion token.
    fn request_assertion_token(&self, principal: &str, scope: &str) -> Result<String> {
        let claims = AssertionClaimsBuilder::default()
            .iss(&self.client_email)
            .aud(&self.token_uri)
            .sub(principal)
            .scope(scope)
            .iat(chrono::Utc::now().timestamp())
            .exp(chrono::Utc::now().timestamp() + 3600)
            .build()?;

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.private_key_id.clone());
        let assertion = jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes())?,
        )?;

        Ok(assertion)
    }

    /// Get an access token from the Google Workspace API on the behalf of a principal.
    ///
    /// * `principal`: The email of the authenticated user requesting this action.
    /// * `scope`: The scope of the assertion token.
    pub async fn get_access_token(&self, principal: &str, scope: &str) -> Result<String> {
        let assertion_token = self.request_assertion_token(principal, scope)?;
        let assertion = AssertionBuilder::default()
            .grant_type(BEARER_TOKEN_GRANT_TYPE)
            .assertion(assertion_token)
            .build()?;

        let res =
            self.http.post(&self.token_uri).body(Vec::<u8>::try_from(assertion)?).send().await?;
        let data = res.json::<GoogleAccessTokenResponse>().await?;

        Ok(data.access_token)
    }
}

#[async_trait]
impl WorkspaceClient for ServiceAccountWorkspaceClient {
    async fn create_user(&self, principal: &str, data: CreateWorkspaceUser) -> Result<()> {
        let scope = "https://www.googleapis.com/auth/admin.directory.user";
        let url = "https://admin.googleapis.com/admin/directory/v1/users";
        let access_token = self.get_access_token(principal, scope).await?;

        self.http
            .post(url)
            .bearer_auth(&access_token)
            .body(Vec::<u8>::try_from(data)?)
            .send()
            .await?;

        Ok(())
    }

    async fn delete_user(&self, principal: &str, email_of_user_to_delete: &str) -> Result<()> {
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

impl Service for ServiceAccountWorkspaceClient {
    fn get_id(&self) -> &'static str {
        "service account workspace client [default]"
    }
}
