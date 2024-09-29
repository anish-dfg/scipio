mod api;
mod api_docs;
mod api_response;
mod errors;
pub mod state;
use std::sync::Arc;

use api_docs::ApiDocs;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::app::state::Services;

/// Build the application router.
///
/// * `ctx`: The application context
pub async fn build(services: Arc<Services>) -> Router<()> {
    let api = api::build(services.clone()).await;

    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDocs::openapi()).path("/rapidoc"))
        .with_state(services)
        .nest("/api", api)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
