use crate::{domain::{Error, EvalEx}, server::{EvalResult, Query, Request}};
use super::request::DeviceDocRequest;

///
/// Extracting incoming messages as [DeviceDocRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceDoc]
pub struct SelectDevDoc {
    // ctx: Box<dyn EvalEx<DevId, Result<JsonCtx, Error>> + Send>,
}
//
//
impl SelectDevDoc {
    ///
    /// Returns [SelectDevDoc] new instance
    pub fn new(
        // ctx: impl EvalEx<DevId, Result<JsonCtx, Error>> + Send + 'static
    ) -> Self {
        Self {
            // ctx: Box::new(ctx),
        }
    }
}
//
//
impl EvalEx<Query<Request>, EvalResult> for SelectDevDoc {
    fn eval(&self, query: Query<Request>) -> EvalResult {
        let error = Error::new("SelectDevDoc", "eval");
        match query.content.get("dev-id") {
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
            None => Err(error.err(format!("data field is not found in {:#?}", query))),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for SelectDevDoc {}
