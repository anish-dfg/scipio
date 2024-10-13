use std::env;

use rstest::fixture;

use crate::{ServiceAccount, ServiceAccountJson};

/// Fixture to create a service account for use in other tests
#[fixture]
pub fn service_account() -> ServiceAccount {
    dotenvy::dotenv().expect("Failed to load .env file");

    let service_account_json_str =
        env::var("WORKSPACE_SERVICE_ACCOUNT_JSON").expect("WORKSPACE_SERVICE_ACCOUNT_JSON not set");

    let service_account_json =
        serde_json::from_str::<ServiceAccountJson>(&service_account_json_str)
            .expect("Failed to deserialize WORKSPACE_SERVICE_ACCOUNT_JSON");

    ServiceAccount::new(service_account_json, 5)
}
