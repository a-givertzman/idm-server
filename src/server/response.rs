use sal_core::error::Error;
use crate::server::Cot;

///
/// The [Response] contains information about the `Request` and `Rply` data
#[derive(Debug, Clone)]
pub struct Response<K, Reply> {
    /// Event id, used internal only to identify incoming request message
    pub event_id: u32,
    /// Name of the [Query], correspond with `query` variant
    pub query_id: K,
    /// Cause of the transmission
    pub cot: Cot,
    /// Response data
    pub data: Reply,
    /// Optional error info
    pub error: Option<Error>,
}
// ///
// /// Error struct for reply, contains an error message
// #[derive(Debug, Clone, Serialize)]
// pub(super) struct ReplyError {
//     pub message: String,
// }
// //
// //
// impl ReplyError {
//     ///
//     /// Returns [ReplyError] new instance
//     pub fn new(err: impl Into<String>) -> Self {
//         Self { message: err.into() }
//     }
// }
