use api_tools::api::message::message::Bytes;
use crate::domain::JsonVal;

///
/// Contains message's `id` & `bytes`
pub struct BytesCtx {
    pub msg_id: u32,
    pub bytes: Bytes,
}
///
/// Contains message's `id` & `JsonVal`
#[derive(Debug, PartialEq)]
pub struct JsonCtx {
    pub msg_id: u32,
    pub value: JsonVal,
    pub is_empty: bool,
}
impl JsonCtx {
    pub fn new(id: u32, value: JsonVal) -> Self {
        Self { msg_id: id, value, is_empty: false }
    }
    pub fn empty() -> Self {
        Self {
            msg_id: 0,
            value: JsonVal::Null,
            is_empty: true,
        }
    }
}
///
/// Contains message's `id` & `Map<String, JsonVal>`
pub struct MapCtx {
    pub msg_id: u32,
    pub map: serde_json::Map<String, JsonVal>,
}
