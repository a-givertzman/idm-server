use api_tools::api::message::message::Bytes;
use serde::Serialize;

///
/// Contains message's `id` & `bytes`
pub(super) struct BytesCtx {
    pub bytes: Bytes,
    pub id: usize,
}
///
/// Contains message's `id` & `serde_json::Value`
pub(super) struct JsonCtx {
    pub value: serde_json::Value,
    pub id: usize,
}
///
/// Contains message's `id` & `Map<String, serde_json::Value>`
pub(super) struct MapCtx {
    pub map: serde_json::Map<String, serde_json::Value>,
    pub id: usize,
}
