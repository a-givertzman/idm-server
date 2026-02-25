use std::fmt::Debug;
use sal_core::{dbg::Dbg, error::Error};
use serde::Serialize;
use crate::server::{Bytes, Content, Cot, Parse, Query, Reply, Response};

///
/// The [Frame] - massage parsed from the socket
#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct Frame<OperationId> {
    /// Frame id, used internal only to identify incoming request message
    pub id: u32,
    /// Name of the [Operation]
    pub operation_id: OperationId,
    /// Cause of the transmission
    pub cot: Cot,
    /// Kind of the content in the `Data` field of  the socket `Message`
    pub content: Content,
    /// Payload data of the [Frame], to be sent to the socket directly 
    pub bytes: Bytes,
}
//
//
impl<QueryId: Debug + Copy> Frame<QueryId> {
    ///
    /// ## Returns [Frame] new instance
    /// - `id` - Idendifier of the received message, take it from the
    /// - `operation_id` - The name of the `Operation`
    /// - `cot` - Cause and diraction of the transmission, beter to use `Cot::reply()` method
    ///     - `Inf` - Information (Informational message, in general sent by backend to the client)
    ///     - `Act` - Activation (Command message, response is not required, optionally my be sent Cot::ActCon / Cot::ActErr)
    ///     - `ActCon` - Activation | Confirmatiom
    ///     - `ActErr` - Activation | Error
    ///     - `Req` - Request (Request message, Client expects response with Cot::ReqCon / Cot::ReqErr)
    ///     - `ReqCon` - Rquest | Confirmatiom reply 
    ///     - `ReqErr` - Rquest | Error reply
    /// - `bytes` - raw bytes to be sent over the socket
    pub fn new(id: u32, operation_id: QueryId, cot: Cot, content: Content, bytes: Bytes) -> Self {
        Self {
            id,
            operation_id,
            cot,
            content, 
            bytes,
        }
    }
    ///
    /// ## Json [Frame] built from object
    /// 
    /// **Message [Content] type will selected automatically depend on the [Reply]**
    /// - [Reply::Empty] - will have [Content::Json]
    /// - [Reply::Bytes] - will have [Content::Bytes]
    /// - [Reply::Error] - will have [Content::Json]
    /// - [Reply::...] - will have [Content::Json]
    /// 
    /// - `dbg` - Parent dbg
    /// - `response` - Prepared [Response] contains `event_id`, `query_id`, `cot` and serializable `Reply` 
    pub fn from(dbg: &Dbg, response: Response<QueryId>) -> Self {
        match &response.reply {
            Reply::Empty => Self {
                id: response.event_id,
                operation_id: response.operation_id,
                cot: response.cot,
                content: Content::Empty,
                bytes: vec![],
            },
            Reply::Error(err) => Self::json(dbg, response.event_id, response.operation_id, response.cot, err),
            Reply::Bytes(_) => Self {
                id: response.event_id,
                operation_id: response.operation_id,
                cot: response.cot,
                content: Content::Bytes,
                bytes: match response.reply {
                    Reply::Bytes(bytes) => bytes,
                    _ => panic!()
                },
            },
            _ => Self::json(dbg, response.event_id, response.operation_id, response.cot, &response.reply),
        }
    }
    ///
    /// Returns [Frame] with [Content::Json]
    fn json(dbg: &Dbg, event_id: u32, operation_id: QueryId, cot: Cot, reply: impl Serialize) -> Self {
        match serde_json::to_vec(&reply) {
            Ok(bytes) => Self {
                id: event_id,
                operation_id,
                cot: cot,
                content: Content::Json,
                bytes,
            },
            Err(err) => Self {
                id: event_id,
                operation_id,
                cot: Self::error_cot(cot),
                content: Content::Json,
                bytes: serde_json::to_vec(
                    &Error::new(Dbg::new(dbg, "Frame"), "json")
                        .pass_with(format!("Can't serialize response {:?}", operation_id), err.to_string())
                        .to_string()
                ).unwrap_or(vec![]),
            },
        }
    }
    ///
    /// Returns Ok [Reply] to current [Request]
    pub fn reply(&self, reply: Reply) -> Response<QueryId> {
        Response {
            event_id: self.id,
            operation_id: self.operation_id,
            cot: self.cot.reply_ok(),
            reply,
        }
    }
    ///
    /// Returns Error [Reply] to current [Request]
    pub fn reply_err(&self, err: impl Into<String>) -> Response<QueryId> {
        Response {
            event_id: self.id,
            operation_id: self.operation_id,
            cot: self.cot.reply_err(),
            reply: Reply::error(err),
        }
    }
    ///
    /// Returns error [Cot] depend on specified
    fn error_cot(cot: Cot) -> Cot {
        match cot {
            Cot::Inf => Cot::Inf,
            Cot::Act => Cot::ActErr,
            Cot::ActCon => Cot::ActErr,
            Cot::ActErr => Cot::ActErr,
            Cot::Req => Cot::ReqErr,
            Cot::ReqCon => Cot::ReqErr,
            Cot::ReqErr => Cot::ReqErr,
        }
    }
    ///
    /// Returns [Operation] built from `Event`
    pub fn operation<T: Parse>(&self) -> Result<T, Error> {
        let query = match self.content {
            Content::Any => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::Bool => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::Bytes =>  T::from_bytes(&self.bytes),
            Content::Duration => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::Empty => Ok(T::empty()),
            Content::F32 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::F64 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::I16 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::I32 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::I64 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::Json => T::from_json(&self.bytes),
            Content::String => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::Timestamp => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::U16 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::U32 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
            Content::U64 => Err(Error::new("Frame", "operation").err(format!("Content {:?} - is not supported", self.content))),
        };
        match query {
            Ok(query) => Ok(query),
            Err(err) => Err(Error::new("Frame", "operation").pass_with(format!("Can't parse operation {:?}", self.operation_id), err)),
        }
    }
}
