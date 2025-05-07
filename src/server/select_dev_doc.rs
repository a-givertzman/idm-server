use sal_core::error::Error;
use crate::{device_info::DevId, domain::Eval};
use super::{request::DeviceDocRequest, JsonCtx, MapCtx};
///
/// Extracting incoming messages as [DeviceDocRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceDoc]
pub(crate) struct SelectDevDoc {
    // ctx: Box<dyn Eval<DevId, Result<JsonCtx, Error>> + Send>,
}
//
//
impl SelectDevDoc {
    ///
    /// Returns [SelectDevDoc] new instance
    pub fn new(
        // ctx: impl Eval<DevId, Result<JsonCtx, Error>> + Send + 'static
    ) -> Self {
        Self {
            // ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for SelectDevDoc {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("SelectDevDoc", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: DeviceDocRequest = data;
                        // match self.ctx.eval(DevId(req.id)) {
                        //     Ok(value) => Ok(value),
                        //     Err(err) => Err(error.pass(err.to_string())),
                        // }
                        Err(error.err("Not implemented"))
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
unsafe impl Send for SelectDevDoc {}
