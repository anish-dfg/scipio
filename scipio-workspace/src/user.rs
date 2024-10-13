//! This module defines the user entities that ServiceAccount relies on.
//!
//! Complete documentation of these entities may be found
//! [here](https://developers.google.com/admin-sdk/directory/reference/rest)

// There's no point documenting here because everything can be found at the link in the file
// header.
#![allow(missing_docs)]
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The hash function used to hash the user's password.
///
/// If rounds are specified as part of the prefix of the user's password, they must be 10,000 or
/// fewer.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HashFunction {
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "SHA-1")]
    Sha1,
    #[serde(rename = "crypt")]
    Crypt,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct UserName {
    #[builder(setter(into))]
    pub given_name: String,
    #[builder(setter(into))]
    pub family_name: String,
    #[builder(setter(into), default = "None")]
    pub full_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Custom,
    Home,
    Other,
    Work,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Email {
    pub address: String,
    pub primary: bool,
    #[serde(rename = "type")]
    pub _type: EmailType,
    pub custom_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalIdType {
    Account,
    Custom,
    Customer,
    LoginId,
    Network,
    Organization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct ExternalId {
    pub value: String,
    #[serde(rename = "type")]
    pub _type: ExternalIdType,
    pub custom_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    AdminAssistant,
    Assistant,
    Brother,
    Child,
    Custom,
    DomesticPartner,
    DottedLineManager,
    #[serde(rename = "exec_assistant")]
    ExecutiveAssistant,
    Father,
    Friend,
    Manager,
    Mother,
    Parent,
    Partner,
    ReferredBy,
    Relative,
    Sister,
    Spouse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Relation {
    pub value: String,
    pub _type: RelationType,
    pub custom_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Custom,
    Home,
    Other,
    Work,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Address {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub custom_type: Option<String>,
    pub extended_address: Option<String>,
    pub formatted: Option<String>,
    pub locality: Option<String>,
    pub po_box: Option<String>,
    pub postal_code: Option<String>,
    pub primary: bool,
    pub region: Option<String>,
    pub source_is_structured: bool,
    pub street_address: Option<String>,
    #[serde(rename = "type")]
    pub _type: AddressType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Organization {
    pub name: String,
    pub title: String,
    pub primary: bool,
    pub custom_type: Option<String>,
    pub department: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub location: Option<String>,
    pub symbol: Option<String>,
    pub cost_center: Option<String>,
    pub full_time_equivalents: Option<i32>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhoneType {
    Assistant,
    Callback,
    Car,
    CompanyMain,
    Custom,
    GrandCentral,
    Home,
    HomeFax,
    Isdn,
    Main,
    Mobile,
    Other,
    OtherFax,
    Pager,
    Radio,
    Telex,
    TtyTdd,
    Work,
    WorkFax,
    WorkMobile,
    WorkPager,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Phone {
    pub value: String,
    pub primary: bool,
    pub _type: PhoneType,
    pub custom_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguagePreference {
    Preferred,
    NotPreferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Language {
    pub language_code: Option<String>,
    pub custom_language: Option<String>,
    pub preference: Option<LanguagePreference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PosixAccountOsType {
    Linux,
    Unspecified,
    Windows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct PosixAccount {
    pub account_id: String,
    pub gecos: String,
    pub gid: usize,
    pub home_directory: String,
    pub operating_system_type: PosixAccountOsType,
    pub primary: bool,
    pub shell: String,
    pub system_id: String,
    pub uid: usize,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct SshPublicKey {
    pub expiration_time_usec: isize,
    pub fingerprint: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoteContentType {
    #[serde(rename = "text_plain")]
    Text,
    #[serde(rename = "text_html")]
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Note {
    pub value: String,
    pub content_type: NoteContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebsiteType {
    AppInstallPage,
    Blog,
    Custom,
    Ftp,
    Home,
    HomePage,
    Other,
    Profile,
    Reservations,
    Resume,
    Work,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Website {
    pub primary: bool,
    pub custom_type: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    Custom,
    Default,
    Desk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Location {
    pub area: Option<String>,
    pub building_id: Option<String>,
    pub custom_type: Option<String>,
    pub desk_code: Option<String>,
    pub floor_name: Option<String>,
    pub floor_section: Option<String>,
    #[serde(rename = "type")]
    pub _type: LocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeywordType {
    Custom,
    Mission,
    Occupation,
    Outlook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Keyword {
    pub value: String,
    pub _type: KeywordType,
    pub custom_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenderType {
    Female,
    Male,
    Other,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Gender {
    pub address_me_as: String,
    pub custom_gender: Option<String>,
    #[serde(rename = "type")]
    pub _type: GenderType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImProtocol {
    Aim,
    CustomProtocol,
    GoogleTalk,
    Icq,
    Jabber,
    Msn,
    NetMeeting,
    Qq,
    Skype,
    Yahoo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImType {
    Custom,
    Home,
    Other,
    Work,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct Im {
    pub custom_protocol: Option<String>,
    pub custom_type: Option<String>,
    pub im: String,
    pub primary: bool,
    pub protocol: ImProtocol,
    #[serde(rename = "type")]
    pub _type: ImType,
}

/// This struct defines a user in Google Workspace. More documentation about each field can be
/// found [here](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users#User).
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUser {
    pub id: Option<String>,
    pub primary_email: String,
    pub hash_function: Option<HashFunction>,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub is_delegated_admin: bool,
    #[serde(default)]
    pub agreed_to_terms: bool,
    #[serde(default)]
    pub suspended: bool,
    #[serde(default)]
    pub change_password_at_next_login: bool,
    #[serde(default)]
    pub ip_whitelisted: bool,
    pub name: UserName,
    pub kind: String,
    pub etag: String,
    #[serde(default)]
    pub emails: Vec<Email>,
    #[serde(default)]
    pub external_ids: Vec<ExternalId>,
    #[serde(default)]
    pub relations: Vec<Relation>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub is_mailbox_setup: bool,
    pub customer_id: String,
    #[serde(default)]
    pub addresses: Vec<Address>,
    #[serde(default)]
    pub organizations: Vec<Organization>,
    pub last_login_time: Option<String>,
    #[serde(default)]
    pub phones: Vec<Phone>,
    pub suspension_reason: Option<String>,
    pub thumbnail_photo_url: Option<String>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub posix_accounts: Vec<PosixAccount>,
    pub creation_time: String,
    #[serde(default)]
    pub non_editable_aliases: Vec<String>,
    #[serde(default)]
    pub ssh_public_keys: Vec<SshPublicKey>,
    #[serde(default)]
    pub notes: Vec<Note>,
    #[serde(default)]
    pub websites: Vec<Website>,
    #[serde(default)]
    pub locations: Vec<Location>,
    #[serde(default)]
    pub include_in_global_address_list: bool,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    pub deletion_time: Option<String>,
    pub gender: Option<Gender>,
    pub thumbnail_photo_etag: Option<String>,
    #[serde(default)]
    pub ims: Vec<Im>,
    #[serde(default)]
    pub custom_schemas: Vec<Value>,
    #[serde(default, rename = "isEnrolledIn2Sv")]
    pub is_enrolled_in_two_step_verification: bool,
    #[serde(default, rename = "isEnforcedIn2Sv")]
    pub is_enforced_in_two_step_verification: bool,
    #[serde(default)]
    pub archived: bool,
    pub org_unit_path: String,
    pub recovery_email: Option<String>,
    pub recovery_phone: Option<String>,
}

/// Information needed to create a user in Google Workspace.
///
/// `primary_email`, `password`, and `name` are required fields.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
#[serde_with::skip_serializing_none]
pub struct CreateWorkspaceUser {
    #[builder(setter(into))]
    pub primary_email: String,
    pub name: UserName,
    #[builder(setter(into))]
    pub password: String,
    #[builder(setter(into), default = "None")]
    pub hash_function: Option<HashFunction>,
    #[builder(setter(into), default = "None")]
    pub change_password_at_next_login: Option<bool>,
    #[builder(setter(into), default = "None")]
    pub ip_whitelisted: Option<bool>,
    #[builder(setter(into), default = "None")]
    pub emails: Option<Vec<Email>>,
    #[builder(setter(into), default = "None")]
    pub external_ids: Option<Vec<ExternalId>>,
    #[builder(setter(into), default = "None")]
    pub relations: Option<Vec<Relation>>,
    #[builder(setter(into), default = "None")]
    pub addresses: Option<Vec<Address>>,
    #[builder(setter(into), default = "None")]
    pub organizations: Option<Vec<Organization>>,
    #[builder(setter(into), default = "None")]
    pub phones: Option<Vec<Phone>>,
    #[builder(setter(into), default = "None")]
    pub languages: Option<Vec<Language>>,
    #[builder(setter(into), default = "None")]
    pub posix_accounts: Option<Vec<PosixAccount>>,
    #[builder(setter(into), default = "None")]
    pub ssh_public_keys: Option<Vec<SshPublicKey>>,
    #[builder(setter(into), default = "None")]
    pub notes: Option<Vec<Note>>,
    #[builder(setter(into), default = "None")]
    pub websites: Option<Vec<Website>>,
    #[builder(setter(into), default = "None")]
    pub locations: Option<Vec<Location>>,
    #[builder(setter(into), default = "None")]
    pub include_in_global_address_list: Option<bool>,
    #[builder(setter(into), default = "None")]
    pub keywords: Option<Vec<Keyword>>,
    #[builder(setter(into), default = "None")]
    pub gender: Option<Gender>,
    #[builder(setter(into), default = "None")]
    pub ims: Option<Vec<Im>>,
    #[builder(setter(into), default = "None")]
    pub custom_schemas: Option<Vec<Value>>,
    #[builder(setter(into), default = "None")]
    pub archived: Option<bool>,
    #[builder(setter(into), default = "None")]
    pub recovery_email: Option<String>,
    #[builder(setter(into), default = "None")]
    pub recovery_phone: Option<String>,
    #[builder(setter(into))]
    pub org_unit_path: Option<String>,
}

impl TryFrom<CreateWorkspaceUser> for Vec<u8> {
    type Error = anyhow::Error;

    fn try_from(value: CreateWorkspaceUser) -> std::result::Result<Self, Self::Error> {
        Ok(serde_json::to_vec(&value)?)
    }
}
