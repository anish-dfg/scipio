use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

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

/// A field in an Airtable table.
///
/// * `id`: The ID of the field
/// * `_type`: The type of the field
/// * `name`: The name of the field
/// * `description`: The description of the field
/// * `options`: Custom options for the field
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub options: Option<Value>,
}

/// A view in an Airtable table.
///
/// * `id`: The ID of the view
/// * `_type`: The type of the view
/// * `name`: The name of the view
/// * `visible_field_ids`: The IDs of the fields that are visible in the view
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct View {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub visible_field_ids: Option<Vec<String>>,
}

/// A table in an Airtable base.
///
/// * `id`: The ID of the table
/// * `primary_field_id`: The ID of the primary field in the table
/// * `name`: The name of the table
/// * `description`: The description of the table
/// * `fields`: The fields in the table
/// * `views`: The views in the table
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    #[serde(rename = "primaryFieldId")]
    pub primary_field_id: String,
    pub name: String,
    pub description: Option<String>,
    // #[serde(skip_serializing)]
    pub fields: Vec<Field>,
    pub views: Vec<View>,
}

/// A record in an Airtable table.
///
/// * `id`: The ID of the record.
/// * `fields`: The fields of the record.
/// * `created_time`: When the record was created.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record<T> {
    pub id: String,
    pub fields: T,
    pub created_time: String,
}
