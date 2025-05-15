use serde::{Deserialize, Serialize};

use crate::domain::{Error, JsonVal};

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
impl TryFrom<&JsonVal> for Cot {
    type Error = Error;
    ///
    /// Returns [Cot] created from `json::Value`
    fn try_from(value: &JsonVal) -> Result<Self, Self::Error> {
        let error = Error::new("Cot", "try_from");
        match serde_json::from_value(value.to_owned()) {
            Ok(cot) => Ok(cot),
            Err(err) => Err(error.pass_with(format!("Cot can't be parsed from {:#?}", value), err.to_string())),
        }
    }
}