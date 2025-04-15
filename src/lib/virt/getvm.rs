use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct VMInfo {
    #[serde(rename = "ElementName")]
    element_name: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "NumberOfProcessors")]
    number_of_processors: u32,

    #[serde(rename = "MemoryUsage")]
    memory_size_bytes: u64,
}

fn connect_wmi_with_fallback() -> Result<WMIConnection, String> {
    let com_con = COMLibrary::new().map_err(|e| format!("Failed to initialize COM library: {}", e))?;

    println!("Trying ROOT\\virtualization\\v2");
    if let Ok(wmi) = WMIConnection::with_namespace_path("ROOT\\virtualization\\v2", com_con.clone().into()) {
        println!("Connected to v2 namespace");
        return Ok(wmi);
    }

    println!("Falling back to ROOT\\virtualization");
    WMIConnection::with_namespace_path("ROOT\\virtualization", com_con.into())
        .map_err(|e| format!("Failed to connect to WMI (v2 & legacy): {}", e))
}

pub fn get_vm_info(vm_id: &str) -> serde_json::Value {
    println!("Attempting to get VM info for ID: {}", vm_id);

    let wmi_con = match connect_wmi_with_fallback() {
        Ok(wmi) => wmi,
        Err(e) => {
            println!("WMI connection failed: {}", e);
            return json!({ "error": e, "status": "error" });
        }
    };

    let results: Vec<VMInfo> = match wmi_con.raw_query("SELECT * FROM Msvm_ComputerSystem") {
        Ok(res) => {
            println!("Queried VMs: {} found for vm_id {}", res.len(), vm_id);
            res
        }
        Err(e) => {
            println!("WMI query failed: {}", e);
            return json!({
                "error": format!("Failed to query WMI: {}", e), 
                "status": "error"
            });
        }
    };

    if results.is_empty() {
        println!("No VMs found in Hyper-V");
        return json!({ "error": "No VMs found in Hyper-V", "status": "error" });
    }

    if let Some(vm) = results.iter().find(|vm| vm.name.eq_ignore_ascii_case(vm_id)) {
        println!("Found VM: {}", vm.element_name);
        json!({
            "name": vm.element_name,
            "id": vm.name,
            "cpu": { "count": vm.number_of_processors },
            "memory": {
                "total_bytes": vm.memory_size_bytes,
                "total_gb": vm.memory_size_bytes / (1024 * 1024 * 1024)
            }
        })
    } else {
        println!("VM with ID {} not found", vm_id);
        json!({ "error": format!("VM with ID {} not found", vm_id), "status": "error" })
    }
}