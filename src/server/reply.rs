use serde::Serialize;

///
/// The API reply
#[derive(Debug, Clone, Serialize)]
pub(super) struct Reply {
    pub id: u32,
    pub data: serde_json::Value,
    pub error: Option<ReplyError>,
}
///
/// Error struct for reply, contains an error message
#[derive(Debug, Clone, Serialize)]
pub(super) struct ReplyError {
    pub message: String,
}
