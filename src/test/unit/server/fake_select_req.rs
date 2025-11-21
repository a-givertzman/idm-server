use std::fmt::Debug;

use crate::{domain::{Error, EvalEx}, server::{EvalResult, Query, Reply, Request, extract}};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectReq1 {
    ctx: Box<dyn Fn(&str) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectReq1 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(&str) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K: Debug + Copy> EvalEx<Request<K>, EvalResult<K>> for FakeSelectReq1 {
    fn eval(&self, request: Request<K>) -> EvalResult<K> {
        let error = Error::new("FakeSelectReq1", "eval");
        let query = extract!(&request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => Ok(Some(request.reply(Reply::TestString(value)))),
            Err(err) => Err(error.pass(err.to_string())),
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
    ctx: Box<dyn Fn(&str) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectReq2 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(&str) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K: Debug + Copy> EvalEx<Request<K>, EvalResult<K>> for FakeSelectReq2 {
    fn eval(&self, request: Request<K>) -> EvalResult<K> {
        let error = Error::new("FakeSelectReq2", "eval");
        let query = extract!(&request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => Ok(Some(request.reply(Reply::TestString(value)))),
            Err(err) => Err(error.pass(err.to_string())),
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
    ctx: Box<dyn Fn(&str) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectReq3 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(&str) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K: Debug + Copy> EvalEx<Request<K>, EvalResult<K>> for FakeSelectReq3 {
    fn eval(&self, request: Request<K>) -> EvalResult<K> {
        let error = Error::new("FakeSelectReq3", "eval");
        let query = extract!(&request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => Ok(Some(request.reply(Reply::TestString(value)))),
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectReq3 {}
