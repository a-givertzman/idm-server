use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Query}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectReq<R> {
    select: IndexMap<R, Box<dyn EvalEx<Query<R>, EvalResult> + Send>>,
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq> SelectReq<R> {
    ///
    /// Returns [SelectReq] new instance
    pub fn new(select: Vec<(R, Box<dyn EvalEx<Query<R>, EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select
            ),
        }
    }
}
//
//
impl<R: Borrow<R> + Hash + Eq + DeserializeOwned + Debug> EvalEx<(Query<R>, Option<Link>), EvalResult> for SelectReq<R> {
    //
    //
    fn eval(&self, (query, _): (Query<R>, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectReq", "eval");
        match self.select.get(&query.name) {
            Some(eval) => {
                eval.eval(query)
            },
            None => Err(error.err(format!("Request {:?} - is not supported", query.name))),
        }
    }
    //
    //
    fn exit(&self) {
        for (_, e) in &self.select {
            e.exit();
        }
    }
}
//
//
unsafe impl<R> Send for SelectReq<R> {}
