use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(
    title = "Primer API",
    version = "0.1.0",
    description = "REST API for primer."
))]
pub struct ApiDoc;
