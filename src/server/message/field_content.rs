//!
//! # Messages transmitted over socket.
//! 
//! - Data can be encoded using varius data `Kind`, `Size` and payload Data
//! 
//! - Message format
//!     Field name | Start | Kind |  Size  | Data |
//!     ---       |  ---  | ---  |  ---   | ---  |
//!     Data type |  u8   | u8   | u32    | [u8; Size] |
//!     Value     |  22   | StringValue | xxx    | [..., ...]  |
//!     
//!     - Start - Each message starts with SYN (22)
//!     - Kind - The `Kind` of the data stored in the `Data` field, refer to
//!     - Size - The length of the `Data` field in bytes
//!     - Data - Data structured depending on it `Kind`
//! 
//! - `Kind` of data
//!     - 00, Any
//!     - 01, Empty
//!     - 02, Bytes
//!     - 08, Bool
//!     - 16, UInt16
//!     - 17, UInt32
//!     - 18, UInt64
//!     - 24, Int16
//!     - 25, Int32
//!     - 26, Int64
//!     - 32, F32
//!     - 33, F64
//!     - 38, Json
//!     - 40, String
//!     - 48, Timestamp
//!     - 49, Duration
//!     - .., ...
//! 
use sal_core::error::Error;
///
/// Internal Kind of Message
/// - Used for build / parsing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Content {
    Any = Self::ANY as isize,
    Bool = Self::BOOL as isize,
    Bytes = Self::BYTES as isize,
    Duration = Self::DURATION as isize,
    Empty = Self::EMPTY as isize,
    F32 = Self::FLOAT32 as isize,
    F64 = Self::FLOAT64 as isize,
    I16 = Self::INT16 as isize,
    I32 = Self::INT32 as isize,
    I64 = Self::INT64 as isize,
    Json = Self::JSON as isize,
    String = Self::STRING as isize,
    Timestamp = Self::TIMESTAMP as isize,
    U16 = Self::UINT16 as isize,
    U32 = Self::UINT32 as isize,
    U64 = Self::UINT64 as isize,
}
//
//
impl Content {
    const ANY: u8       = 00;
    const BOOL: u8      = 08;
    const BYTES: u8     = 02;
    const DURATION: u8  = 49;
    const EMPTY: u8     = 01;
    const FLOAT32: u8   = 32;
    const FLOAT64: u8   = 33;
    const INT16: u8     = 24;
    const INT32: u8     = 25;
    const INT64: u8     = 26;
    const JSON: u8      = 38;
    const STRING: u8    = 40;
    const TIMESTAMP: u8 = 48;
    const UINT16: u8    = 16;
    const UINT32: u8    = 17;
    const UINT64: u8    = 18;
    // ///
    // /// Returns bytes of the `MessageKund` variant    
    // pub fn to_bytes(&self) -> u8 {
    // }
}
impl TryFrom<&[u8]> for Content {
    type Error = Error;
    ///
    /// Returns [MessageKind] converted from `bytes`
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes {
            [Self::ANY] => Ok(Content::Any),
            [Self::BOOL] => Ok(Content::Bool),
            [Self::BYTES] => Ok(Content::Bytes),
            [Self::DURATION] => Ok(Content::Duration),
            [Self::EMPTY] => Ok(Content::Empty),
            [Self::FLOAT32] => Ok(Content::F32),
            [Self::FLOAT64] => Ok(Content::F64),
            [Self::INT16] => Ok(Content::I16),
            [Self::INT32] => Ok(Content::I32),
            [Self::INT64] => Ok(Content::I64),
            [Self::JSON] => Ok(Content::Json),
            [Self::STRING] => Ok(Content::String),
            [Self::TIMESTAMP] => Ok(Content::Timestamp),
            [Self::UINT16] => Ok(Content::U16),
            [Self::UINT32] => Ok(Content::U32),
            [Self::UINT64] => Ok(Content::U64),
            [..] => Err(Error::new("MessageKind", "from_bytes").err(format!("Wrong or Empty input: {:?}", &bytes[..16]))),
        }
    }
}
impl Into<u8> for Content {
    ///
    /// Returns u8 representation of the [MessageKind]
    fn into(self) -> u8 {
        match self {
            Content::Any => Self::ANY,
            Content::Bool => Self::BOOL,
            Content::Bytes => Self::BYTES,
            Content::Duration => Self::DURATION,
            Content::Empty => Self::EMPTY,
            Content::F32 => Self::FLOAT32,
            Content::F64 => Self::FLOAT64,
            Content::I16 => Self::INT16,
            Content::I32 => Self::INT32,
            Content::I64 => Self::INT64,
            Content::Json => Self::JSON,
            Content::String => Self::STRING,
            Content::Timestamp => Self::TIMESTAMP,
            Content::U16 => Self::UINT16,
            Content::U32 => Self::UINT32,
            Content::U64 => Self::UINT64,
        }
    }
}