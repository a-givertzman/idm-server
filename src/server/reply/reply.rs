use serde::{Deserialize, Serialize};
use crate::device::{DeviceDoc, DeviceInfo};

///
/// Wrapper for all variants of API [Reply]'s
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reply {
    Empty,
    DeviceStream,
    DeviceInfo(DeviceInfo),
    DeviceDoc(DeviceDoc),
}
