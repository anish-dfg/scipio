use serde::{Deserialize, Serialize};
use serde_json::Value;

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
