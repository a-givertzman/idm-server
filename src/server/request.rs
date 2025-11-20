use std::marker::PhantomData;

use sal_core::error::Error;
use serde::de::DeserializeOwned;

use crate::server::{BINCODE_CONFIG, Content, ContentBytes, ContentEmpty, ContentJson, Cot, Query, QueryId, Reply, Response};

///
/// The [Request] contains information about the Event and `Query`
#[derive(Debug, Clone)]
pub struct Request<T> {
    /// Event id, used internal only to identify incoming request message
    pub event_id: u32,
    /// Name of the [Query], correspond with `query` variant
    pub query_id: QueryId,
    /// Cause of the transmission
    pub cot: Cot,
    /// Kind of the [Query] content
    pub content: Content,
    /// [Query] it self
    pub query: Query,
    target: PhantomData<T>,
}
//
//
impl<T> Request<T> {
    ///
    /// Returns [Query] new instance
    /// - `id` - Query id, used internal only to identify incoming request message
    /// - `name` - Name of the [Query]
    /// - `bytes` - 
    pub fn new(event_id: u32, query_id: QueryId, cot: Cot, content: Content, query: Query) -> Self {
        Self {
            event_id,
            query_id,
            cot,
            content,
            query,
            target: PhantomData,
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
///
/// Converting `Message` payload bytes into the requested type `T`,
/// which depends on the kind of message `Content`
pub trait Parse<T> {
    fn parse(&self, bytes: &[u8]) -> Result<T, Error>;
}
//
//
impl<T: bincode::Decode<()>> Parse<T> for Request<ContentBytes> {
    fn parse(&self, bytes: &[u8]) -> Result<T, Error> {
        match self.content {
            Content::Bytes => bincode::decode_from_slice(bytes, BINCODE_CONFIG)
                .map(|(v, _)| v)
                .map_err(|err| Error::new("Request<ContentBytes>", "parse").pass(err.to_string())),
            _ => todo!(),
        }
    }
}
//
//
impl Parse<()> for Request<ContentEmpty> {
    fn parse(&self, _: &[u8]) -> Result<(), Error> {
        Ok(())
    }
}
//
//
impl<T: DeserializeOwned> Parse<T> for Request<ContentJson> {
    fn parse(&self, bytes: &[u8]) -> Result<T, Error> {
        serde_json::from_slice(&bytes)
            .map_err(|err| Error::new("Request<ContentJson>", "parse").pass(err.to_string()))
    }
}
