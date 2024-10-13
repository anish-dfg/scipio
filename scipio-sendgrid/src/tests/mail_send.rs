use anyhow::Result;
use rstest::rstest;

use super::fixtures::sendgrid;
use crate::entities::{
    AddressBuilder, MailBuilder, MailContentBuilder, MailContentMime, PersonalizationBuilder,
};
use crate::Sendgrid;

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
pub async fn test_send_mail(sendgrid: Sendgrid) -> Result<()> {
    let personalization = PersonalizationBuilder::default()
        .to(vec![AddressBuilder::default()
            .email("anish@developforgood.org")
            .name("Anish".to_owned())
            .build()?])
        .build()?;

    let mail = MailBuilder::default()
        .from(
            AddressBuilder::default()
                .email("pantheon@developforgood.org")
                .name("Pantheon".to_owned())
                .build()?,
        )
        .personalizations(vec![personalization])
        .subject("Test email")
        .content(vec![MailContentBuilder::default()
            .value("This is a test email from Pantheon".to_owned())
            .mime_type(MailContentMime::Plain)
            .build()?])
        .build()?;

    sendgrid.send_mail(mail).await?;
    Ok(())
}
