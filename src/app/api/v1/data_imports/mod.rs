mod airtable;
mod controllers;
mod requests;
mod responses;

use std::sync::Arc;

use axum::extract::FromRef;
use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use requests::ImportAirtableBase;
use utoipa::OpenApi;

use crate::app::api::middleware::make_rbac;
use crate::app::state::Services;

#[derive(OpenApi)]
#[openapi(
    paths(controllers::import_airtable_base, controllers::list_available_airtable_bases),
    components(schemas(ImportAirtableBase))
)]
pub struct DataImportsApi;

pub struct ImportServices {
    pub storage_layer: Arc<dyn crate::services::storage::StorageService>,
    pub airtable: Arc<dyn crate::services::airtable::AirtableService>,
}

impl FromRef<Arc<Services>> for ImportServices {
    fn from_ref(ctx: &Arc<Services>) -> Self {
        Self { storage_layer: ctx.storage_layer.clone(), airtable: ctx.airtable.clone() }
    }
}

pub async fn build(ctx: Arc<Services>) -> Router<()> {
    let guard1 = make_rbac(vec!["read:available-bases".to_owned()]).await;

    let import_airtable_base = routing::post(controllers::import_airtable_base);
    let list_available_airtable_bases = routing::get(controllers::list_available_airtable_bases);

    Router::new()
        .route("/airtable/base/:base_id", import_airtable_base)
        .route("/airtable/available-bases", list_available_airtable_bases)
        .route_layer(from_fn_with_state(ctx.clone(), guard1))
        .with_state(ctx.clone())
}
