use serde::{Deserialize, Serialize};

///
/// Cose of transmission of the TCP message
/// - `Act` - Activation (Client -> Server)
/// - `ActCon` - Activation confirmation (Server -> Client)
/// - `ActErr` - Activation error (Server -> Client)
/// - `Rec` - Recquest (Client -> Server)
/// - `RecCon` - Request confirmation, contains reply (Server -> Client)
/// - `RecErr` - Request error (Server -> Client)
/// - `Inf` - Information message (Server -> Client)
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