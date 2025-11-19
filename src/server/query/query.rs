use serde::{Deserialize, Serialize};

///
/// Wrapper for all variants of API [Query]'s
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    DeviceStream(DeviceStreamQuery),
    DeviceInfo(DeviceInfoQuery),
    DeviceDoc(DeviceDocQuery),
}
///
/// Request for `DeviceStream`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStreamQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfoQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDocQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
