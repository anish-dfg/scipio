use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::app::api::v1::data_exports::requests::ExportUsersToWorkspaceRequest;

pub struct EmailPolicy {
    pub add_unique_numeric_suffix: bool,
    pub separator: Option<String>,
    pub use_first_and_last_name: bool,
}

impl EmailPolicy {
    pub fn build_volunteer_email(&self, first_name: &str, last_name: &str) -> String {
        let mut base = if self.use_first_and_last_name {
            format!(
                "{}{}{}",
                first_name.to_lowercase(),
                self.separator.as_ref().unwrap_or(&"".to_string()),
                last_name.to_lowercase()
            )
        } else {
            first_name.to_lowercase()
        };

        if self.add_unique_numeric_suffix {
            let mut rng = rand::thread_rng();
            let suffix = rng.gen_range(10..100);

            base.push_str(&suffix.to_string());
        }

        let mut cleaned = base.chars().filter(|c| c.is_alphanumeric()).collect::<String>();

        cleaned.push_str("@volunteer.developforgood.org");
        cleaned
    }
}

pub struct PasswordPolicy {
    pub change_password_at_next_login: bool,
    pub generated_password_length: u8,
}

impl PasswordPolicy {
    pub fn generate_password(&self) -> String {
        if !(8..=64).contains(&self.generated_password_length) {
            log::warn!(
                "Password length must be between 8 and 64 characters. Defaulting to 8 characters."
            );
        }
        match self.generated_password_length {
            // minimum, and default, is 8. max is 64
            0..=7 | 65.. => rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
            8..=64 => rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(self.generated_password_length as usize)
                .map(char::from)
                .collect::<String>(),
        }
    }
}

impl From<&ExportUsersToWorkspaceRequest> for EmailPolicy {
    fn from(request: &ExportUsersToWorkspaceRequest) -> Self {
        Self {
            add_unique_numeric_suffix: request.add_unique_numeric_suffix,
            separator: request.separator.clone(),
            use_first_and_last_name: request.use_first_and_last_name,
        }
    }
}

impl From<&ExportUsersToWorkspaceRequest> for PasswordPolicy {
    fn from(request: &ExportUsersToWorkspaceRequest) -> Self {
        Self {
            change_password_at_next_login: request.change_password_at_next_login,
            generated_password_length: request.generated_password_length,
        }
    }
}
