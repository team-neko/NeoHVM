use utoipa::OpenApi;

use crate::routes::tool;

/// OpenAPI documentation definition
#[derive(OpenApi)]
#[openapi(
    paths(
        tool::root,
    ),
    info(
        title = "AL-1S",
        description = "A beautifully designed Hyper-V management changer.",
        version = "0.0.0"
    )
)]
pub struct ApiDoc;
