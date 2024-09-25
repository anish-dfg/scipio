use serde::{Deserialize, Serialize};

use crate::services::storage::entities::VolunteerDetails;

#[derive(Debug, Serialize, Deserialize)]
pub struct Volunteers {
    pub volunteers: Vec<VolunteerDetails>,
}
