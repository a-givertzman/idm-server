use serde::{Deserialize, Serialize};

///
/// List of API requiests
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Request {
    DeviceStream,
    DeviceInfo,
    DeviceDoc,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfoRequest {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceDocRequest {
    #[serde(rename="devId")]
    pub dev_id: String,
}
