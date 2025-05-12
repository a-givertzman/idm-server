use bincode::{Decode, Encode};

///
/// The [Event] contains message id and bytes to be sent
#[derive(Debug, Clone, Encode, Decode)]
pub(super) struct Event {
    pub id: u32,
    pub bytes: Vec<u8>,
}
