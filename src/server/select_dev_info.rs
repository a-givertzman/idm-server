use sal_core::error::Error;
use serde::{Deserialize, Serialize};
use crate::{device_info::DevId, domain::Eval};
use super::{JsonCtx, MapCtx};
///
/// Extracting incoming messages as [DeviceInfoRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceInfo]
pub(crate) struct SelectDevInfo {
    ctx: Box<dyn Eval<DevId, Result<JsonCtx, Error>> + Send>,
}
//
//
impl SelectDevInfo {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Eval<DevId, Result<JsonCtx, Error>> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for SelectDevInfo {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("SelectDevInfo", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: DeviceInfoRequest = data;
                        match self.ctx.eval(DevId(req.id)) {
                            Ok(value) => Ok(value),
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
unsafe impl Send for SelectDevInfo {}
///
/// Request for `DeviceInfo`
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeviceInfoRequest {
    pub id: u32,
}
