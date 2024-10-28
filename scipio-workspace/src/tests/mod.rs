mod fixtures;

use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rstest::rstest;

use super::tests::fixtures::service_account;
use crate::user::{CreateWorkspaceUserBuilder, UserNameBuilder};
use crate::ServiceAccount;

#[cfg(feature = "integration")]
#[rstest]
#[tokio::test]
async fn test_create_and_delete_user(service_account: ServiceAccount) -> Result<()> {
    let random_suffix =
        rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>();

    let email = format!("test-anish-{random_suffix}@developforgood.org");

    let data = CreateWorkspaceUserBuilder::default()
        .primary_email(email)
        .name(
            UserNameBuilder::default()
                .given_name("Anish".to_owned())
                .family_name("Sinha".to_owned())
                .build()?,
        )
        .password("password".to_owned())
        .change_password_at_next_login(true)
        .recovery_email("anish@developforgood.org".to_owned())
        .org_unit_path("/Programs/PantheonUsers".to_owned())
        .build()?;

    let user = service_account.create_user("anish@developforgood.org", data).await?;
    dbg!(&user);

    service_account.delete_user("anish@developforgood.org", &user.primary_email).await?;

    Ok(())
}
