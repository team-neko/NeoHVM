mod routes;
mod docs;
mod lib;

use axum::{routing::{get, get_service}, Json, Router};
use tokio::net::TcpListener;
use tracing_subscriber;
//use utoipa_scalar::{Scalar, Servable};
use utoipa::OpenApi;
use tower_http::services::ServeDir;
use docs::ApiDoc;

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Define the API routes
    let api_routes = Router::new()
        .merge(routes::routes())
        .route("/api-docs/openapi.json", get(openapi))
    ;

    // Serve static files from ./web/dist/
    let static_files = get_service(ServeDir::new("./web/dist"));

    // Combine static file serving and API routes
    let app = Router::new()
        .nest("/api", api_routes) // Mount the API routes under /api
        .fallback_service(static_files);   // Use fallback to serve static files

    println!("🚀 Server running at http://127.0.0.1:5300");
    println!("📖 API docs at http://127.0.0.1:5300/api/scalar");

    let listener = TcpListener::bind("0.0.0.0:5300").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
