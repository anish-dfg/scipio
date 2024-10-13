use std::env;

use rstest::fixture;

use crate::Sendgrid;

#[fixture]
pub fn sendgrid() -> Sendgrid {
    dotenvy::dotenv().expect("Failed to load .env file");
    let api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set");

    Sendgrid::new(&api_key, 3).expect("Failed to create Sendgrid instance")
}
