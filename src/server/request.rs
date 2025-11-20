use crate::server::{Cot, Query, QueryId, Reply, Response};

///
/// The [Request] contains information about the Event and `Query`
#[derive(Debug, Clone)]
pub struct Request {
    /// Event id, used internal only to identify incoming request message
    pub event_id: u32,
    /// Name of the [Query], correspond with `query` variant
    pub query_id: QueryId,
    /// Cause of the transmission
    pub cot: Cot,
    /// [Query] it self
    pub query: Query,
}
//
//
impl Request {
    ///
    /// Returns [Query] new instance
    /// - `id` - Query id, used internal only to identify incoming request message
    /// - `name` - Name of the [Query]
    /// - `bytes` - 
    pub fn new(event_id: u32, query_id: QueryId, cot: Cot, query: Query) -> Self {
        Self {
            event_id,
            query_id,
            cot,
            query,
        }
    }
    ///
    /// Returns Ok [Reply] to current [Request]
    pub fn reply(&self, reply: Reply) -> Response {
        Response {
            event_id: self.event_id,
            query_id: self.query_id,
            cot: self.cot.reply_ok(),
            reply,
        }
    }
    ///
    /// Returns Error [Reply] to current [Request]
    pub fn reply_err(&self, err: impl Into<String>) -> Response {
        Response {
            event_id: self.event_id,
            query_id: self.query_id,
            cot: self.cot.reply_err(),
            reply: Reply::error(err),
        }
    }
}
pub trait Decode {
    fn decode<T>(&self) -> T;
}
impl Decode for Request {
    fn decode<T>(&self) -> T {
        todo!()
    }
}