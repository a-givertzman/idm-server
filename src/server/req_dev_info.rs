use sal_core::error::Error;
use serde::{Deserialize, Serialize};
use crate::{device_info::DeviceInfo, domain::Eval};
use super::{JsonCtx, MapCtx};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub(crate) struct ReqDevInfo {
    
}
//
//
impl ReqDevInfo {
    ///
    /// Returns [SortByX] new instance
    pub fn new() -> Self {
        Self {
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for ReqDevInfo {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("ReqDevInfo", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: DeviceInfoRequest = data;
                        let reply = DeviceInfo {
                            id: req.id,
                            name: "Device Name".to_owned(),
                            model: "Device Model".to_owned(),
                            serial: "Device Serial".to_owned(),
                        };
                        match serde_json::to_value(reply) {
                            Ok(value) => Ok(JsonCtx { value, id: input.id }),
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("data field is not found in {:#?}", input.map))),
        }
    }
}
//
//
unsafe impl Send for ReqDevInfo {}
///
/// Request for `DeviceInfo`
#[derive(Debug, Deserialize)]
struct DeviceInfoRequest {
    pub id: usize,
}
