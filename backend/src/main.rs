use axum::{
    routing::get,
    Json, Router,
};
use serde_json::{self, json};
use tokio::net::TcpListener;
use tracing_subscriber;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

// Define the OpenAPI schema
#[derive(OpenApi)]
#[openapi(
    paths(root),
    info(
        title = "AL-1S",
        description = "A beautifully designed Hyper-V management changer.",
        version = "0.0.0"
    )
)]
struct ApiDoc;

// GET /
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Return status and timestamp", body = serde_json::Value)
    )
)]
async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "timestamp": chrono::Local::now().timestamp_millis(),
    }))
}

// Handler to serve the OpenAPI JSON spec
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::fmt::init();

    // Build app
    let app = Router::new()
        .route("/", get(root))
        .route("/api-docs/openapi.json", get(openapi))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi())); // Add Scalar UI

    // Bind directly to 127.0.0.1:3000
    println!("🚀 Server running at http://127.0.0.1:5300");
    println!("📖 API docs at http://127.0.0.1:5300/scalar");

    let listener = TcpListener::bind("127.0.0.1:5300").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}