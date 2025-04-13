// get hostname
#[allow(dead_code)]
pub fn get_hostname() -> String {
    let mut hostname = String::new();
    if let Ok(host) = std::process::Command::new("hostname").output() {
        if let Ok(host_str) = String::from_utf8(host.stdout) {
            hostname = host_str.trim().to_string();
        }
    }
    hostname
}