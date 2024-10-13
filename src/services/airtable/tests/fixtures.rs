use std::env;

use rstest::fixture;
use scipio_airtable::Airtable;

#[fixture]
pub fn airtable() -> Airtable {
    dotenvy::dotenv().expect("error loading environment variables");
    let api_token = env::var("AIRTABLE_API_TOKEN").expect("missing AIRTABLE_API_TOKEN variable");
    Airtable::new(&api_token, 5).expect("error creating Airtable client")
}
