use axum::{Router, Json, response::IntoResponse, http::StatusCode};
use axum::routing::get;
use serde_json::json;
use std::time::Instant;
use crate::lib::host::tool::get_hostname;

pub fn routes() -> Router {
    Router::new().route("/hostname", get(hostname))
}

#[utoipa::path(
    get,
    path = "/hostname",
    responses(
        (status = 200, description = "Get Host Name", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
/// Get Host Name
pub async fn hostname() -> impl IntoResponse {
    let start_time = Instant::now();
    let hostname = get_hostname();
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    (StatusCode::OK, Json(json!({
        "hostname": hostname,
        "time": duration.as_millis(),
        "status": "success"
    })))
}
