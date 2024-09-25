use serde::{Deserialize, Serialize};

use crate::services::storage::entities::ProjectCycle;

/// Cycle response from the API.
///
/// * `cycles`: The cycles returned from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclesResponse {
    pub cycles: Vec<ProjectCycle>,
}
