mod controllers;
mod import_airtable_base;
mod requests;
mod responses;

use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::{routing, Router};
use import_airtable_base::{
    import_task as import_airtable_base_task, ImportTaskParams as ImportAirtableBaseTaskParams,
};
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
