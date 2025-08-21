use std::{borrow::Borrow, hash::Hash, fmt::Debug};
use crate::{domain::{Error, EvalEx, JsonVal, Link}, server::{EvalResult, Query}};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectAct1 {
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectAct1 {
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
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<(Query<R>, Option<Link>), EvalResult> for FakeSelectAct1 {
    fn eval(&self, (query, link): (Query<R>, Option<Link>)) -> EvalResult {
        let error = Error::new("FakeSelectAct1", "eval");
        let key = "data";
        match query.content.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => {
                                link.unwrap().send(
                                    serde_json::to_string(&value).unwrap(),
                                ).unwrap();
                                Ok(None)
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.content))),
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
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectAct2 {
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
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<(Query<R>, Option<Link>), EvalResult> for FakeSelectAct2 {
    fn eval(&self, (query, link): (Query<R>, Option<Link>)) -> EvalResult {
        let error = Error::new("FakeSelectAct2", "eval");
        let key = "data";
        match query.content.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => {
                                link.unwrap().send(
                                    serde_json::to_string(&value).unwrap(),
                                ).unwrap();
                                Ok(None)
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.content))),
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
    ctx: Box<dyn Fn(String) -> Result<JsonVal, Error> + Send>,
}
//
//
impl FakeSelectAct3 {
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
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<(Query<R>, Option<Link>), EvalResult> for FakeSelectAct3 {
    fn eval(&self, (query, link): (Query<R>, Option<Link>)) -> EvalResult {
        let error = Error::new("FakeSelectAct3", "eval");
        let key = "data";
        match query.content.get(key) {
            Some(val) => {
                match serde_json::from_value(val.to_owned()) {
                    Ok(data) => {
                        match (self.ctx)(data) {
                            Ok(value) => {
                                link.unwrap().send(
                                    serde_json::to_string(&value).unwrap(),
                                ).unwrap();
                                Ok(None)
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("'{key}' field is not found in {:#?}", query.content))),
        }
    }
    fn exit(&self) {}
}
//
//
unsafe impl Send for FakeSelectAct3 {}
