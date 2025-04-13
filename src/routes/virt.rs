// Todo : Fix the issue with the wmi
use axum::{Router, Json, response::IntoResponse, http::StatusCode};
use axum::routing::get;
use serde_json::json;
use std::time::Instant;
use crate::lib::virt::list::get_vm_list;

pub fn routes() -> Router {
    Router::new().route("/list", get(list))
}

#[utoipa::path(
    get,
    path = "/list",
    responses(
        (status = 200, description = "Return Hyper-V virtual machines List", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
/// List Hyper-V virtual machines
pub async fn list() -> impl IntoResponse {
    let start_time = Instant::now();
    let vm_list = get_vm_list();
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);

    (StatusCode::OK, Json(json!({
        "vms": vm_list,
        "time": duration.as_millis(),
        "status": "success"
    })))
}
