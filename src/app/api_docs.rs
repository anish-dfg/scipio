use utoipa::OpenApi;

use super::api::Api;

/// The API documentation for the entire application.
///
/// This struct generates OpenAPI documentation for the entire application. Currently, we serve the
/// docs using `Rapidoc` on the `/rapidoc` endpoint.
#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/api", api = Api)
    ),
)]
pub struct ApiDocs;
