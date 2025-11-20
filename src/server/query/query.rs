use sal_core::error::Error;
use serde::{Deserialize, Serialize};

use crate::server::{BINCODE_CONFIG, QueryId};

///
/// Wrapper for all variants of API [Query]'s
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    DeviceStream(DeviceStreamQuery),
    DeviceInfo(DeviceInfoQuery),
    DeviceDoc(DeviceDocQuery),
    BytesExample(BytesExampleQuery),
}
//
//
impl Query {
    ///
    /// Returns [Query] parsed from JSON `bytes`
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        match serde_json::from_slice(&bytes) {
            Ok(query) => Ok(query),
            Err(err) => Err(Error::new("Query", "from_json").pass(err.to_string())),
        }
    }
    ///
    /// Returns [Query] parsed from raw `bytes` using `bincode::Decode`
    pub fn from_bytes(query_id: QueryId, bytes: &[u8]) -> Result<Self, Error> {
        let error = Error::new("Query", "from_json");
        match query_id {
            QueryId::BytesExample => Self::bin_decode(&error, &bytes).map(Query::BytesExample),
            _ => serde_json::from_slice(&bytes).map_err(|err| error.pass(err.to_string())),
        }
    }
    ///
    /// 
    fn bin_decode<T: bincode::Decode<()>>(error: &Error, bytes: &[u8]) -> Result<T, Error> {
        bincode::decode_from_slice(bytes, BINCODE_CONFIG).map(|(v, _)| v).map_err(|err| error.pass(err.to_string()))
    }
}
///
/// Request for `DeviceStream`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStreamQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfoQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request for `DeviceInfo`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDocQuery {
    #[serde(rename="devId")]
    pub dev_id: String,
}
///
/// Request example with `Content::Bytes`
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct BytesExampleQuery {
    val: f64,
    name: String,
    data: Vec<Pt>,
}
#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct Pt {
    x: f64,
    y: f64,
}