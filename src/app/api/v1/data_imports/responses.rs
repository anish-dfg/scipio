use serde::{Deserialize, Serialize};

use crate::services::airtable::entities::bases::Base;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableBases {
    pub bases: Vec<Base>,
}
