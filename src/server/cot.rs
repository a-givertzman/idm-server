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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cot {
    Act,
    ActCon,
    Req,
    ReqCon,
    Inf,
}