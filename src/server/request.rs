use serde::Deserialize;

///
/// List of API requiests
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Request {
    DeviceInfo,
    DeviceDoc,
}