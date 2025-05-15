use crate::{device_info::DevId, domain::{Error, Eval, JsonVal}};
use super::{request::DeviceInfoRequest, JsonCtx, MapCtx};

///
/// Extracting incoming messages as [DeviceInfoRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceInfo]
pub(crate) struct SelectDevInfo {
    ctx: Box<dyn Eval<DevId, Result<JsonVal, Error>> + Send>,
}
//
//
impl SelectDevInfo {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Eval<DevId, Result<JsonVal, Error>> + Send + 'static) -> Self {
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
                        match self.ctx.eval(DevId(req.dev_id)) {
                            Ok(value) => Ok(JsonCtx::new(input.msg_id, value)),
                            Err(err) => Err(error.pass(err)),
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
