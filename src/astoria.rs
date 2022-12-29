use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AstprocdMessage {
    status: String,
    astoria_version: String,
    code_status: String,
    disk_info: DiskInfo,
    pub pid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiskInfo {
    uuid: String,
    mount_path: String,
    disk_type: String,
}
