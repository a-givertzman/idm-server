use api_tools::api::message::{fields::{FieldId, FieldSize}, message::{Bytes, Message}, message_kind::MessageKind};
///
/// Message transmitted over TCP stream
pub type TcpMessage = Message<(FieldId, MessageKind, FieldSize, Bytes)>;
