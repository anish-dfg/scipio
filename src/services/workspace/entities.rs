//! This module defines the entities that `WorkspaceClient` implementations rely on.
//!
//! Complete documentation of these entities may be found
//! [here](https://developers.google.com/admin-sdk/directory/reference/rest)

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct defines a user in Google Workspace. More documentation about each field can be
/// found [here](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User).
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUser {
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub primary_email: String,
    pub name: Name,
    pub is_admin: bool,
    pub is_delegated_admin: bool,
    pub last_login_time: String,
    pub creation_time: String,
    pub agreed_to_terms: bool,
    pub suspended: bool,
    pub archived: bool,
    pub change_password_at_next_login: bool,
    pub ip_whitelisted: bool,
    pub emails: Vec<Email>,
    pub languages: Vec<Language>,
    pub non_editable_aliases: Vec<String>,
    pub customer_id: String,
    pub org_unit_path: String,
    pub is_mailbox_setup: bool,
    #[serde(rename = "isEnrolledIn2Sv")]
    pub is_enrolled_in2sv: bool,
    #[serde(rename = "isEnforcedIn2Sv")]
    pub is_enforced_in2sv: bool,
    pub include_in_global_address_list: bool,
    pub recovery_email: Option<String>,
}

/// A user's name in Google Workspace. More documentation about each field can be found
/// [here](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#UserName).
#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct Name {
    #[builder(setter(into))]
    pub given_name: String,
    #[builder(setter(into))]
    pub family_name: String,
    #[builder(setter(into), default = "None")]
    pub full_name: Option<String>,
}

/// An email address in Google Workspace. This is a JSON object. Currently Google Workspace only
/// defines four possible fields:
///
/// - `address`: The email address.
/// - `type`: The type of email address. Possible values are "custom", "home", "work", "other".
/// - `primary`: Whether this is the primary email address.
/// - `custom_type`: A custom type for the email address.
///
/// `custom_type` isn't represented in this struct. It isn't used, but it should be added if
/// needed. I missed it in the original implementation.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub address: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub primary: Option<bool>,
}

/// A language in Google Workspace. This is a JSON object. Currently Google Workspace only defines
/// two fields:
///
/// - `languageCode`: The language code.
/// - `preference`: The preference for the language.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub language_code: String,
    pub preference: String,
}

/// This represents the response from the Google Workspace API when you request a list of users.
/// More documentation on each field can be found
/// [here](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users/list)
#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceUserData {
    pub trigger_event: String,
    pub kind: String,
    pub etag: String,
    pub users: Vec<WorkspaceUser>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
}

/// This struct defines the information necessary to create a user in Google Workspace. More
/// information about each field can be found
/// [here](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceUser {
    #[builder(setter(into))]
    pub primary_email: String,
    pub name: Name,
    #[builder(setter(into))]
    pub password: String,
    pub change_password_at_next_login: bool,
    #[builder(setter(into))]
    pub recovery_email: String,
    #[builder(setter(into), default = "None")]
    pub recovery_phone: Option<String>,
    #[builder(setter(into), default = "\"/Programs/PantheonUsers\".to_string()")]
    pub org_unit_path: String,
}

impl TryFrom<CreateWorkspaceUser> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: CreateWorkspaceUser) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_vec(&value)?)
    }
}
