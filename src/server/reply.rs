use serde::Serialize;
use crate::domain::JsonVal;

///
/// The API reply
#[derive(Debug, Clone, Serialize)]
pub(super) struct Reply {
    // pub id: u32,
    pub data: JsonVal,
    pub error: Option<ReplyError>,
}
///
/// Error struct for reply, contains an error message
#[derive(Debug, Clone, Serialize)]
pub(super) struct ReplyError {
    pub message: String,
}
