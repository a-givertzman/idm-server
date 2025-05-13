use sal_core::error::Error;
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
//
//
impl TryFrom<&serde_json::Value> for Cot {
    type Error = Error;
    ///
    /// Returns [Cot] created from `serde_json::Value`
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let error = Error::new("Cot", "try_from");
        match serde_json::from_value(value.to_owned()) {
            Ok(cot) => Ok(cot),
            Err(err) => Err(error.pass_with(format!("Cot can't be parsed from {:#?}", value), err.to_string())),
        }
    }
}