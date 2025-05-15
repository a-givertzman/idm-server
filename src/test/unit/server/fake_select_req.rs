use serde_json::json;
use crate::{domain::{Error, Eval, JsonVal}, server::{JsonCtx, MapCtx}};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectReq1 {
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectReq1 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<JsonVal, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq1 {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("FakeSelectReq1", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: String = data;
                        match (self.ctx)(req) {
                            Ok(value) => Ok(JsonCtx::new(input.msg_id, json!(value))),
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
unsafe impl Send for FakeSelectReq1 {}
///
/// Fake Req2 handler
pub(crate) struct FakeSelectReq2 {
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectReq2 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<JsonVal, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq2 {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("FakeSelectReq2", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: String = data;
                        match (self.ctx)(req) {
                            Ok(value) => Ok(JsonCtx::new(input.msg_id, json!(value))),
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
unsafe impl Send for FakeSelectReq2 {}
///
/// Fake Req3 handler
pub(crate) struct FakeSelectReq3 {
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectReq3 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<JsonVal, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq3 {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("FakeSelectReq3", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: String = data;
                        match (self.ctx)(req) {
                            Ok(value) => Ok(JsonCtx::new(input.msg_id, json!(value))),
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
unsafe impl Send for FakeSelectReq3 {}
