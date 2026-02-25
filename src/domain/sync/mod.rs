mod link;
mod hub;
mod link_send;

use std::time::Duration;

///
/// Default timeout to await `recv`` operation, 300 ms
pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(10);


pub use link_send::LinkSend;
pub use hub::Hub;
pub use link::Link;
