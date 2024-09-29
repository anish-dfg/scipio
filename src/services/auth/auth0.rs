//! This module contains the implementation of the `Auth0` authenticator.

use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Authenticator;
use crate::services::auth::{AuthData, UserData};
use crate::services::Service;

/// Configuration for the Auth0 authenticator.
///
/// This struct is deserialized from the OpenID configuration endpoint for the Auth0 tenant. It's
/// important to note that not all of these fields are standard. However, the ones that are
/// standard are documented in the
/// [OpenID Connect Discovery
/// 1.0](https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata).
///
/// The fields that are not standard are custom fields added by Auth0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth0Configuration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub device_authorization_endpoint: String,
    pub userinfo_endpoint: String,
    pub mfa_challenge_endpoint: String,
    pub jwks_uri: String,
    pub registration_endpoint: String,
    pub revocation_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub request_uri_parameter_supported: bool,
    pub request_parameter_supported: bool,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub backchannel_logout_supported: Option<bool>,
    pub backchannel_logout_session_supported: Option<bool>,
    pub end_session_endpoint: String,
}

/// The `Auth0` authenticator.
///
/// * `tenant_base_uri`: The base URI for the Auth0 tenant
/// * `audiences`: The audiences that the authenticator will accept
/// * `configuration`: The configuration for the Auth0 tenant
/// * `http`: A reqwest client
#[derive(Debug, Clone)]
pub struct Auth0 {
    #[allow(unused)]
    // TODO: Determine if this is actually needed and whether we can remove it
    pub tenant_base_uri: String,
    pub audiences: Vec<String>,
    pub configuration: Auth0Configuration,
    http: Client,
}

/// User info returned by the Auth0 authenticator.
///
/// Information about each field can be found in the
/// [OpenID Connect Core 1.0, Section 5.1](https://openid.net/specs/openid-connect-core-1_0.html#StandardClaims)
///
/// This struct is not exhaustive and only includes a subset of allowed fields. More can be added as needed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub sub: String,
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub email: String,
    pub email_verified: bool,
}

/// Audience for the token.
///
/// This is represented as a string or a list of strings so we need to use an enum to handle both
/// cases
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Aud {
    Single(String),
    Multiple(Vec<String>),
}

/// Auth0 access token claims.
///
/// This is a superset of the JWT standard claims defined in
/// [RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519#section-4.1)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AccessTokenClaims {
    pub email: String,
    pub iss: String,
    pub sub: String,
    pub aud: Aud,
    pub iat: i64,
    pub exp: i64,
    pub scope: String,
    pub azp: String,
    // NOTE: Custom claim added by Auth0 rule
    pub permissions: Vec<String>,
}

/// Auth0 authentication data.
///
/// * `email`: The email of the user
/// * `token`: The user's JWT
/// * `permissions`: The user's permissions
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Auth0AuthData {
    pub email: String,
    pub token: String,
    pub permissions: Vec<String>,
}

impl Auth0 {
    const DISCOVERY_ENDPOINT_SUFFIX: &'static str = "/.well-known/openid-configuration";
    // TODO: Determine if this needed
    #[allow(unused)]
    const USERINFO_ENDPOINT: &'static str = "/userinfo";

    /// Create a new `Auth0` authenticator.
    ///
    /// * `tenant_base_uri`: The base URI for the Auth0 tenant
    /// * `audiences`: The audiences that the authenticator will accept
    pub async fn new(tenant_base_uri: &str, audiences: Vec<String>) -> Result<Self> {
        let http = Client::new();
        let discovery_endpoint = tenant_base_uri.to_owned() + Self::DISCOVERY_ENDPOINT_SUFFIX;
        let res = http
            .get(&discovery_endpoint)
            .send()
            .await
            .context("fetch auth0 openid configuration")?;

        let configuration = res
            .json::<Auth0Configuration>()
            .await
            .context("deserialize auth0 openid configuration")?;

        Ok(Self { tenant_base_uri: tenant_base_uri.into(), audiences, configuration, http })
    }
}

#[async_trait]
impl Authenticator for Auth0 {
    async fn authenticate(&self, token: &str) -> Result<AuthData> {
        let http = &self.http;
        let jwks_uri = &self.configuration.jwks_uri;

        let res = http.get(jwks_uri).send().await.context("fetch auth0 jwks")?;

        let jwks = res.json::<JwkSet>().await.context("deserialize auth0 jwks")?;

        let header = jsonwebtoken::decode_header(token).context("decode auth0 token header")?;

        let Some(kid) = header.kid else { bail!("missing key id") };

        let Some(jwk) = jwks.keys.into_iter().find(|jwk| match jwk.common.key_id.clone() {
            Some(jwk_id) => jwk_id == kid,
            None => false,
        }) else {
            bail!("no matching key id")
        };

        let decoded = match jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let (n, e) = (rsa.n, rsa.e);
                let mut validator = Validation::new(header.alg);
                validator.set_audience(&self.audiences);
                let Ok(decoded) = jsonwebtoken::decode::<Value>(
                    token,
                    &DecodingKey::from_rsa_components(&n, &e)
                        .context("create decoding key from rsa components")?,
                    &validator,
                ) else {
                    bail!("unable to verify token signature");
                };
                decoded
            }
            _ => bail!("unimplemented algorithm"),
        };

        let token_claims = serde_json::from_value::<AccessTokenClaims>(decoded.claims)?;

        Ok(AuthData::Auth0(Auth0AuthData {
            token: token.to_owned(),
            permissions: token_claims.permissions,
            email: token_claims.email,
        }))
    }

    async fn user_info(&self, token: &str) -> Result<UserData> {
        let http = &self.http;
        let userinfo_endpoint = &self.configuration.userinfo_endpoint;

        let res = http.get(userinfo_endpoint).bearer_auth(token).send().await?;

        let userinfo = res.json::<UserInfo>().await?;

        Ok(UserData::Auth0(userinfo))
    }
}

impl Service for Auth0 {
    fn get_id(&self) -> &'static str {
        "auth0 [default]"
    }
}
