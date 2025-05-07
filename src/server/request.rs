use serde::{Deserialize, Serialize};

///
/// List of API requiests
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Request {
    DeviceInfo,
    DeviceDoc,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeviceInfoRequest {
    pub id: u32,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeviceDocRequest {
    pub id: u32,
}
