use utoipa::OpenApi;

use super::api::v1::V1Api;
use super::api::TopLevelApi;

/// The API documentation for the entire application.
///
/// This struct generates OpenAPI documentation for the entire application. Currently, we serve the
/// docs using `Rapidoc` on the `/rapidoc` endpoint.
#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/api", api = TopLevelApi),
        (path="/api/v1", api = V1Api)
    ),
)]
// TODO: Move this down into the `api` module to better structure the code
pub struct ApiDocs;
