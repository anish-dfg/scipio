use serde::{Deserialize, Serialize};

use crate::services::airtable::entities::record::Record;

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
