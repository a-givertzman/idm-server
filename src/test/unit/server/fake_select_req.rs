use std::{borrow::Borrow, hash::Hash, fmt::Debug};
use serde_json::json;
use crate::{domain::{Error, EvalEx}, server::{EvalResult, Request}};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectReq1 {
    ctx: Box<dyn Fn(String) -> EvalResult + Send>,
}
//
//
impl FakeSelectReq1 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> EvalResult + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<Request<R>, EvalResult> for FakeSelectReq1 {
    fn eval(&self, query: Request<R>) -> EvalResult {
        let error = Error::new("FakeSelectReq1", "eval");
        let key = "data";
        match query.query.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => Ok(Some(json!(value))),
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.query))),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectReq1 {}
///
/// Fake Req2 handler
pub(crate) struct FakeSelectReq2 {
    ctx: Box<dyn Fn(String) -> EvalResult + Send>,
}
//
//
impl FakeSelectReq2 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> EvalResult + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<Request<R>, EvalResult> for FakeSelectReq2 {
    fn eval(&self, query: Request<R>) -> EvalResult {
        let error = Error::new("FakeSelectReq2", "eval");
        let key = "data";
        match query.query.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => Ok(Some(json!(value))),
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.query))),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectReq2 {}
///
/// Fake Req3 handler
pub(crate) struct FakeSelectReq3 {
    ctx: Box<dyn Fn(String) -> EvalResult + Send>,
}
//
//
impl FakeSelectReq3 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> EvalResult + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<Request<R>, EvalResult> for FakeSelectReq3 {
    fn eval(&self, query: Request<R>) -> EvalResult {
        let error = Error::new("FakeSelectReq3", "eval");
        let key = "data";
        match query.query.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => Ok(Some(json!(value))),
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.query))),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectReq3 {}
