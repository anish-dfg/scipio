use derive_builder::Builder;
use scipio_workspace::user::{CreateWorkspaceUser, CreateWorkspaceUserBuilder, UserNameBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct CreateWorkspaceVolunteer {
    #[builder(setter(into))]
    pub primary_email: String,
    #[builder(setter(into))]
    pub first_name: String,
    #[builder(setter(into))]
    pub last_name: String,
    #[builder(setter(into))]
    pub password: String,
    #[builder(setter(into))]
    pub recovery_email: String,
}

impl TryFrom<CreateWorkspaceVolunteer> for CreateWorkspaceUser {
    type Error = anyhow::Error;

    fn try_from(value: CreateWorkspaceVolunteer) -> std::result::Result<Self, Self::Error> {
        let user = CreateWorkspaceUserBuilder::default()
            .name(
                UserNameBuilder::default()
                    .given_name(value.first_name)
                    .family_name(value.last_name)
                    .build()?,
            )
            .password(value.password)
            .change_password_at_next_login(true)
            .primary_email(value.primary_email)
            .recovery_email(value.recovery_email)
            .org_unit_path("/Programs/PantheonUsers".to_owned())
            .build()?;

        Ok(user)
    }
}
