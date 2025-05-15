use api_tools::api::message::message::Bytes;
use crate::device_info::DevId;

///
/// Contains message's `id` & `bytes`
pub struct BytesCtx {
    pub id: DevId,
    pub bytes: Bytes,
}
///
/// Contains message's `id` & `serde_json::Value`
#[derive(Debug, PartialEq)]
pub struct JsonCtx {
    pub id: DevId,
    pub value: serde_json::Value,
    pub is_empty: bool,
}
impl JsonCtx {
    pub fn new(id: DevId, value: serde_json::Value) -> Self {
        Self { id, value, is_empty: false }
    }
    pub fn empty() -> Self {
        Self {
            id: DevId("".to_owned()),
            value: serde_json::Value::Null,
            is_empty: true,
        }
    }
}
///
/// Contains message's `id` & `Map<String, serde_json::Value>`
pub struct MapCtx {
    pub id: DevId,
    pub map: serde_json::Map<String, serde_json::Value>,
}
