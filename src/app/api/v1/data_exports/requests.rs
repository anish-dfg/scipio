use serde::{Deserialize, Serialize};

use crate::services::storage::entities::VolunteerDetails;

/// Request to export users to a workspace.
///
/// * `add_unique_numeric_suffix`: Whether to add a unique 2-digit numeric suffix to the email
///   handle.
/// * `change_password_at_next_login`: Whether to force users to change their password at their
///   next login.
/// * `generated_password_length`: The length of the generated password.
/// * `separator`: The separator to use for the email handle (between the first and last names).
/// * `skip_users_on_conflict`: Whether to skip users on conflict. THIS IS CURRENTLY IGNORED.
/// * `use_first_and_last_name`: Whether to use the first and last names for the email handle.
/// * `volunteers`: The volunteers to export.
// TODO: Either remove `skip_users_on_conflict` or implement it. If it is implemented, its
// semantics need to be crystal clear.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUsersToWorkspaceRequest {
    pub add_unique_numeric_suffix: bool,
    pub change_password_at_next_login: bool,
    pub generated_password_length: u8,
    pub separator: Option<String>,
    pub skip_users_on_conflict: bool,
    pub use_first_and_last_name: bool,
    pub volunteers: Vec<VolunteerDetails>,
}
