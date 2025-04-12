use axum::{routing::get, Json, Router};
use serde_json::json;

pub fn routes() -> Router {
    Router::new().route("/", get(root))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Return status and timestamp", body = serde_json::Value)
    )
)]
/// Root
pub async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "status": "success",
        "timestamp": chrono::Local::now().timestamp_millis(),
    }))
}
