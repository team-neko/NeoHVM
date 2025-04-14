// src/routes.rs
use axum::{Router, Json, response::IntoResponse, http::StatusCode, extract::Path};
use axum::routing::get;
use serde_json::json;
use std::time::Instant;
use crate::lib::virt::list::get_vm_list;
use crate::lib::virt::getvm::get_vm_info;

pub fn routes() -> Router {
    Router::new()
        .route("/list", get(list))
        .route("/vm/:id", get(get_vm))
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
    let duration = start_time.elapsed();

    (StatusCode::OK, Json(json!({
        "data": vm_list,
        "time": duration.as_millis(),
        "status": "success"
    })))
}

#[utoipa::path(
    get,
    path = "/vm/:id",
    responses(
        (status = 200, description = "Return Hyper-V virtual machine information", body = serde_json::Value),
        (status = 404, description = "Virtual machine not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[axum::debug_handler]
/// Get Hyper-V virtual machine information
pub async fn get_vm(Path(id): Path<String>) -> impl IntoResponse {
    let start_time = Instant::now();
    let vm_info = get_vm_info(&id);
    let duration = start_time.elapsed();

    if vm_info.get("error").is_some() {
        (StatusCode::NOT_FOUND, Json(json!({
            "error": vm_info.get("error"),
            "time": duration.as_millis(),
            "status": "error"
        })))
    } else {
        (StatusCode::OK, Json(json!({
            "data": vm_info,
            "time": duration.as_millis(),
            "status": "success"
        })))
    }
}
