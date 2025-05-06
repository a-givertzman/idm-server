use serde::{Deserialize, Serialize};

///
/// List of API requiests
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub(super) enum Req {
    DeviceInfo,
    DeviceDoc,
}