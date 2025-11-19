use sal_core::error::Error;

use crate::server::{Cot, Response};

///
/// The [Request] contains information about the Event and `Query`
#[derive(Debug, Clone)]
pub struct Request<K, Query> {
    /// Event id, used internal only to identify incoming request message
    pub event_id: u32,
    /// Name of the [Query], correspond with `query` variant
    pub query_id: K,
    /// Cause of the transmission
    pub cot: Cot,
    /// [Query] it self
    pub query: Query,
}
//
//
impl<K: Copy, Query> Request<K, Query> {
    ///
    /// Returns [Query] new instance
    /// - `id` - Query id, used internal only to identify incoming request message
    /// - `name` - Name of the [Query]
    /// - `bytes` - 
    pub fn new(event_id: u32, query_id: K, cot: Cot, query: Query) -> Self {
        Self {
            event_id,
            query_id,
            cot,
            query,
        }
    }
    ///
    /// Returns Ok [Reply] to current [Request]
    pub fn reply<R>(&self, data: R) -> Response<K, R> {
        Response {
            event_id: self.event_id,
            query_id: self.query_id,
            cot: self.cot.reply_ok(),
            data: data,
            error: None,
        }
    }
    ///
    /// Returns Error [Reply] to current [Request]
    pub fn reply_err<R: Default>(&self, err: Error) -> Response<K, R> {
        Response {
            event_id: self.event_id,
            query_id: self.query_id,
            cot: self.cot.reply_err(),
            data: Default::default(),
            error: Some(err),
        }
    }
}
