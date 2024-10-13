use std::env;

use anyhow::Result;
use chrono::Utc;
use rstest::{fixture, rstest};
use scipio_sendgrid::Sendgrid;
use tera::Context;

use crate::services::mail::{
    EmailClient, OnboardingEmailParams, OnboardingEmailParamsBuilder, TEMPLATES,
};

#[fixture]
pub fn sendgrid() -> Sendgrid {
    dotenvy::dotenv().expect("error loading environment variables");
    let api_key = env::var("SENDGRID_API_KEY").expect("missing SENDGRID_API_KEY variable");

    Sendgrid::new(&api_key, 8).expect("error constructing client")
}

#[test]
pub fn test_render_template() {
    let mut context = Context::new();
    context.insert("name", "Anish");
    context.insert("email", "anish@developforgood.org");
    context.insert("temporaryPassword", "password123");

    let template = TEMPLATES.render("email/onboard.html", &context).unwrap();
    println!("{template}");
}

#[rstest]
#[tokio::test]
pub async fn test_send_onboarding_email(sendgrid: Sendgrid) -> Result<()> {
    let params = OnboardingEmailParams {
        first_name: "Anish".to_owned(),
        last_name: "Sinha".to_owned(),
        email: "anishsinha0128@gmail.com".to_owned(),
        workspace_email: "anish@developforgood.org".to_owned(),
        temporary_password: "password123".to_owned(),
        send_at: None,
    };

    sendgrid.send_onboarding_email(params).await?;

    Ok(())
}

#[rstest]
#[tokio::test]
pub async fn test_send_scheduled_onboarding_email(sendgrid: Sendgrid) -> Result<()> {
    let now = Utc::now().timestamp() as u64;

    let params = OnboardingEmailParamsBuilder::default()
        .first_name("Anish")
        .last_name("Sinha")
        .email("anish@developforgood.org")
        .workspace_email("anish@dichromatic.com")
        .temporary_password("password123")
        .send_at(now + 120)
        .build()?;

    sendgrid.send_onboarding_email(params).await?;

    Ok(())
}
