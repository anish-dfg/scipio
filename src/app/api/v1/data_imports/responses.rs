use scipio_airtable::base_data::entities::Base;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableBases {
    pub bases: Vec<Base>,
}
