use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Query, Request, extract}};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectAct1 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct1 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K> EvalEx<(Request<K>, Option<Link>), EvalResult<K>> for FakeSelectAct1 {
    fn eval(&self, (request, link): (Request<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("FakeSelectAct1", "eval");
        let query = extract!(request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => {
                link.unwrap().send(
                    serde_json::to_string(&value).unwrap(),
                ).unwrap();
                Ok(None)
            }
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectAct1 {}
///
/// Fake Req2 handler
pub(crate) struct FakeSelectAct2 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct2 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K> EvalEx<(Request<K>, Option<Link>), EvalResult<K>> for FakeSelectAct2 {
    fn eval(&self, (request, link): (Request<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("FakeSelectAct2", "eval");
        let query = extract!(request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => {
                link.unwrap().send(
                    serde_json::to_string(&value).unwrap(),
                ).unwrap();
                Ok(None)
            }
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectAct2 {}
///
/// Fake Req3 handler
pub(crate) struct FakeSelectAct3 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct3 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl<K> EvalEx<(Request<K>, Option<Link>), EvalResult<K>> for FakeSelectAct3 {
    fn eval(&self, (request, link): (Request<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("FakeSelectAct3", "eval");
        let query = extract!(request.query, Query::TestString).unwrap();
        match (self.ctx)(query) {
            Ok(value) => {
                link.unwrap().send(
                    serde_json::to_string(&value).unwrap(),
                ).unwrap();
                Ok(None)
            }
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectAct3 {}
