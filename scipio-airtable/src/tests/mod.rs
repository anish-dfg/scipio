mod bases;
mod fixtures;
mod records;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    #[serde(rename = "Record Id")]
    pub record_id: String,
    #[serde(rename = "Customer Id")]
    pub customer_id: String,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Phone 1")]
    pub phone_1: String,
    #[serde(rename = "Phone 2")]
    pub phone_2: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Subscription Date")]
    pub subscription_date: String,
    #[serde(rename = "Website")]
    pub website: String,
}

impl Customer {
    pub fn field_names() -> Vec<&'static str> {
        vec![
            "Record Id",
            "Customer Id",
            "First Name",
            "Last Name",
            "Company",
            "City",
            "Country",
            "Phone 1",
            "Phone 2",
            "Email",
            "Subscription Date",
            "Website",
        ]
    }
}
