pub mod airtable;
pub mod auth;
pub mod mail;
pub mod storage;
pub mod url;
pub mod workspace;

pub trait Service: Send + Sync {
    fn get_id(&self) -> &'static str;
}
