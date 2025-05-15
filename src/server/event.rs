use bincode::{Decode, Encode};

///
/// The [Event] contains message id and bytes to be sent
#[derive(Debug, Clone, Encode, Decode)]
pub struct Event {
    pub msg_id: u32,
    pub bytes: Vec<u8>,
}
