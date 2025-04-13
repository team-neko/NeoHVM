use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VM {
    element_name: String,
    name: String,
    enabled_state: u16,
}

pub fn get_vm_list() -> serde_json::Value {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::with_namespace_path("ROOT\\virtualization\\v2", com_con.into()).unwrap();
    
    let results: Vec<VM> = wmi_con.raw_query("SELECT * FROM Msvm_ComputerSystem").unwrap();
    
    let vms = results.iter().filter(|vm| vm.element_name != vm.name).map(|vm| {
        json!({
            "name": vm.element_name,
            "id": vm.name,
            "state": vm.enabled_state
        })
    }).collect::<Vec<_>>();
    
    json!({
        "vms": vms
    })
}
