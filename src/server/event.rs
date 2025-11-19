use crate::server::{Query, QueryId, Reply, Request, Response};

///
/// The [Event] contains the name and data bytes to be sent
#[derive(Debug, Clone)]
pub enum Event {
    /// Used to send request to the client
    Request(Request<QueryId, Query>),
    /// Used to send reply to the client
    Response(Response<QueryId, Reply>),
}
