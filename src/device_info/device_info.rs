use serde::Serialize;

///
/// Reply to `DeviceInfo` request
#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    pub id: usize,
    pub name: String,
    pub model: String,
    pub serial: String,
}