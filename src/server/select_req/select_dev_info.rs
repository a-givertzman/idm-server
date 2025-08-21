use crate::{device_info::DevId, domain::{Error, EvalEx}, server::{EvalResult, Query, Request}};

///
/// Extracting incoming messages as [DeviceInfoRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceInfo]
pub(crate) struct SelectDevInfo {
    ctx: Box<dyn EvalEx<DevId, EvalResult> + Send>,
}
//
//
impl SelectDevInfo {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl EvalEx<DevId, EvalResult> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl EvalEx<Query<Request>, EvalResult> for SelectDevInfo {
    fn eval(&self, query: Query<Request>) -> EvalResult {
        let error = Error::new("SelectDevInfo", "eval");
        let key = "dev-id";
        match query.content.get(key) {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(dev_id) => {
                        match self.ctx.eval(DevId(dev_id)) {
                            Ok(value) => Ok(value),
                            Err(err) => Err(error.pass(err)),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.content))),
        }
    }
    fn exit(&self) {
        self.ctx.exit();
    }
}
//
//
unsafe impl Send for SelectDevInfo {}
