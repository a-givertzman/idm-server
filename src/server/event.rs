use bincode::{Decode, Encode};

///
/// The [Event] contains the name and data bytes to be sent
#[derive(Debug, Clone, Encode, Decode)]
pub struct Event {
    /// Message id, used to identify incoming request message
    pub msg_id: u32,
    /// Payload data of the [Event]
    pub bytes: Vec<u8>,
}
//
//
impl Event {
    ///
    /// Returns [Event] new instance
    /// - `id` - Message id, used to identify incoming request message
    /// - `bytes` - 
    pub fn new(id: u32, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            msg_id: id,
            bytes: bytes.into(),
        }
    }
}
