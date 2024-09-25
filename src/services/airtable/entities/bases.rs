use serde::{Deserialize, Deserializer, Serialize};

/// The permission level for a base.
///
/// More information about this definition can be found
/// [here](https://airtable.com/developers/web/api/list-bases). We have an `Other` variant to
/// ensure forward compatibility with new permission levels if Airtable introduces them.
#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PermissionLevel {
    None,
    Read,
    Comment,
    Edit,
    Create,
    Other(String),
}

impl<'de> Deserialize<'de> for PermissionLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        match s.as_ref() {
            "none" => Ok(PermissionLevel::None),
            "read" => Ok(PermissionLevel::Read),
            "comment" => Ok(PermissionLevel::Comment),
            "edit" => Ok(PermissionLevel::Edit),
            "create" => Ok(PermissionLevel::Create),
            _ => Ok(PermissionLevel::Other(s)),
        }
    }
}

/// A base in Airtable.
///
/// * `id`: The ID of the base
/// * `name`: The name of the base
/// * `permission_level`: The permission level the API token used to fetch the base has on the
///   base.
///
/// More information about this definition can be found
/// [here](https://airtable.com/developers/web/api/list-bases)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base {
    pub id: String,
    pub name: String,
    #[serde(rename = "permissionLevel")]
    pub permission_level: PermissionLevel,
}
