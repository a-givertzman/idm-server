use crate::{device::DevId, domain::{Error, EvalEx}, server::{EvalResult, Query, Request}};

///
/// Extracting incoming messages as [DeviceDocRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceDoc]
pub struct SelectDevDoc {
    ctx: Box<dyn EvalEx<DevId, EvalResult> + Send>,
}
//
//
impl SelectDevDoc {
    ///
    /// Returns [SelectDevDoc] new instance
    pub fn new(
        ctx: impl EvalEx<DevId, EvalResult> + Send + 'static
    ) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl EvalEx<Query<Request>, EvalResult> for SelectDevDoc {
    fn eval(&self, query: Query<Request>) -> EvalResult {
        let error = Error::new("SelectDevDoc", "eval");
        match query.content.get("dev-id") {
            Some(dev_id) => {
                match serde_json::from_value(dev_id.to_owned()) {
                    Ok(dev_id) => {
                        match self.ctx.eval(DevId(dev_id)) {
                            Ok(value) => Ok(value),
                            Err(err) => Err(error.pass(err.to_string())),
                        }
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
