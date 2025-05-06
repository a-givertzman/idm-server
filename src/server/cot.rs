use serde::{Deserialize, Serialize};

///
/// Cose of transmission of the TCP message
/// - `Act` - Activation
/// - `ActCon` - Activation confirmation
/// - `ActErr` - Activation error
/// - `Rec` - Recquest
/// - `RecCon` - Request confirmation, contains reply
/// - `RecErr` - Request error
/// - `Inf` - Information message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
pub enum Cot {
    Act,
    ActCon,
    ActErr,
    Req,
    ReqCon,
    ReqErr,
    Inf,
}