use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ImportAirtableBase {
    pub name: String,
    pub description: String,
}
