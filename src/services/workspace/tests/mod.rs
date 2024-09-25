use std::env;

use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rstest::{fixture, rstest};

use crate::services::workspace::entities::{
    CreateWorkspaceUser, CreateWorkspaceUserBuilder, NameBuilder,
};
use crate::services::workspace::service_account::ServiceAccountWorkspaceClient;
use crate::services::workspace::WorkspaceClient;

#[fixture]
pub fn service_account_workspace_client() -> ServiceAccountWorkspaceClient {
    dotenvy::dotenv().expect("error loading environment variables");
    let private_key_id =
        env::var("WORKSPACE_PRIVATE_KEY_ID").expect("missing WORKSPACE_PRIVATE_KEY_ID variable");
    let private_key =
        env::var("WORKSPACE_PRIVATE_KEY").expect("missing WORKSPACE_PRIVATE_KEY variable");
    let workspace_client_email =
        env::var("WORKSPACE_CLIENT_EMAIL").expect("missing WORKSPACE_CLIENT_EMAIL variable");

    ServiceAccountWorkspaceClient::new(
        &workspace_client_email,
        &private_key_id,
        &private_key,
        "https://oauth2.googleapis.com/token",
        5,
    )
}

#[rstest]
#[tokio::test]
pub async fn test_service_account_create_user(
    service_account_workspace_client: ServiceAccountWorkspaceClient,
) -> Result<()> {
    let data = CreateWorkspaceUserBuilder::default()
        .primary_email("anish_test2@developforgood.org")
        .name(NameBuilder::default().given_name("Anish").family_name("Sinha").build()?)
        .password("password")
        .change_password_at_next_login(true)
        .recovery_email("anish@developforgood.org")
        .recovery_phone(Some("+12027656192".to_owned()))
        .build()
        .expect("error building user");

    service_account_workspace_client.create_user("anish@developforgood.org", data).await?;

    Ok(())
}

#[ignore]
#[rstest]
#[tokio::test]
pub async fn test_service_account_create_300_users(
    service_account_workspace_client: ServiceAccountWorkspaceClient,
) -> Result<()> {
    let data = (0..300)
        .map(|_| {
            let random_first_name = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>();
            let random_last_name = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect::<String>();
            CreateWorkspaceUserBuilder::default()
                .primary_email(format!(
                    "test-{random_first_name}-{random_last_name}@developforgood.org"
                ))
                .name(
                    NameBuilder::default()
                        .given_name(random_first_name.clone())
                        .family_name(random_last_name.clone())
                        .build()
                        .expect("error building name"),
                )
                .password("password")
                .change_password_at_next_login(true)
                .recovery_email("anish@developforgood.org")
                .recovery_phone(Some("+12027656192".to_owned()))
                .build()
                .expect("error building user")
        })
        .collect::<Vec<CreateWorkspaceUser>>();

    for user in data {
        service_account_workspace_client.create_user("anish@developforgood.org", user).await?;
    }

    Ok(())
}

#[rstest]
#[tokio::test]
pub async fn test_service_account_delete_user(
    service_account_workspace_client: ServiceAccountWorkspaceClient,
) -> Result<()> {
    let random_first_name =
        rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>();
    let random_last_name =
        rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect::<String>();

    let primary_email = format!("dtest-{random_first_name}-{random_last_name}@developforgood.org");

    let data = CreateWorkspaceUserBuilder::default()
        .primary_email(primary_email.clone())
        .name(
            NameBuilder::default()
                .given_name(random_first_name.clone())
                .family_name(random_last_name.clone())
                .build()
                .expect("error building name"),
        )
        .password("password")
        .change_password_at_next_login(true)
        .recovery_email("anish@developforgood.org")
        .recovery_phone(Some("+12027656192".to_owned()))
        .build()
        .expect("error building user");

    service_account_workspace_client
        .create_user("anish@developforgood.org", data)
        .await
        .expect("error creating user");

    service_account_workspace_client
        .delete_user("anish@developforgood.org", &primary_email)
        .await
        .expect("error deleting user");

    Ok(())
}
