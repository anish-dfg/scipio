mod api;
mod api_docs;
mod api_response;
pub mod context;
mod errors;
use std::sync::Arc;

use api_docs::ApiDocs;
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use crate::app::context::Context;

/// Build the application router.
///
/// * `ctx`: The application context
pub async fn build(ctx: Arc<Context>) -> Router<()> {
    let api = api::build(ctx.clone()).await;

    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    Router::new()
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDocs::openapi()).path("/rapidoc"))
        .with_state(ctx)
        .nest("/api", api)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
