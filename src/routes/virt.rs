// Todo : Fix the issue with the wmi
use axum::{Router, Json};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wmi::{WMIConnection, COMLibrary};
use tokio::task::spawn_blocking;

pub fn routes() -> Router {
    Router::new().route("/list", get(list))
}

#[utoipa::path(
    get,
    path = "/list",
    responses(
        (status = 200, description = "Return Hyper-V virtual machines List", body = serde_json::Value)
    )
)]

/// List Hyper-V virtual machines
pub async fn list() -> Json<serde_json::Value> {
    // Define struct to deserialize WMI query results
    #[derive(Deserialize)]
    struct MsvmComputerSystem {
        #[serde(rename = "ElementName")]
        element_name: String,
        #[serde(rename = "Name")]
        name: String,
    }

    // Define struct to serialize VM info into JSON
    #[derive(Serialize)]
    struct VmInfo {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "ID")]
        id: String,
    }

    // Execute WMI query in a blocking context since wmi crate is synchronous
    let vms_result = spawn_blocking(|| -> Result<Vec<VmInfo>, wmi::WMIError> {
        // Connect to Hyper-V WMI namespace
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::with_namespace_path("root\\virtualization\\v2", com_lib)?;
        // Query virtual machines
        let results: Vec<MsvmComputerSystem> = wmi_con.query::<MsvmComputerSystem>()?;
        // Map results to VmInfo structs
        let vms: Vec<VmInfo> = results.into_iter().map(|vm| VmInfo {
            name: vm.element_name,
            id: vm.name,
        }).collect();
        Ok(vms)
    }).await;

    // Handle the result and construct JSON response
    match vms_result {
        Ok(Ok(vms)) => Json(json!({
            "status": "success",
            "timestamp": chrono::Local::now().timestamp_millis(),
            "vms": vms,
        })),
        Ok(Err(e)) => Json(json!({
            "status": "error",
            "message": e.to_string(),
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Failed to spawn blocking task: {}", e),
        })),
    }
}