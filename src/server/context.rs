use api_tools::api::message::message::Bytes;

///
/// Contains message's `id` & `bytes`
pub(super) struct BytesCtx {
    pub id: u32,
    pub bytes: Bytes,
}
///
/// Contains message's `id` & `serde_json::Value`
pub(super) struct JsonCtx {
    pub id: u32,
    pub value: serde_json::Value,
}
///
/// Contains message's `id` & `Map<String, serde_json::Value>`
pub(super) struct MapCtx {
    pub id: u32,
    pub map: serde_json::Map<String, serde_json::Value>,
}
