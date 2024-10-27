use serde::{Deserialize, Serialize};

use super::entities::{Base, Record, Table};

/// Base response from the Airtable API.
///
/// * `offset`: The offset to start listing bases from if we need to fetch more.
/// * `bases`: The bases returned from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBasesResponse {
    pub offset: Option<String>,
    pub bases: Vec<Base>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaResponse {
    pub tables: Vec<Table>,
}

/// Response from the Airtable API for listing records.
///
/// * `records`: The records returned from the API.
/// * `offset`: The offset to start listing records from if we need to fetch more. (a pagination
///   token).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListRecordsResponse<T> {
    pub records: Vec<Record<T>>,
    pub offset: Option<String>,
}

pub type GetRecordResponse<T> = Record<T>;
