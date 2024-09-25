use serde::{Deserialize, Serialize};

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
