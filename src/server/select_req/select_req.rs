use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Request}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectReq<K, Q> {
    select: IndexMap<K, Box<dyn EvalEx<Request<K, Q>, EvalResult> + Send>>,
}
//
//
impl<K: std::hash::Hash + std::cmp::Eq, Q> SelectReq<K, Q> {
    ///
    /// Returns [SelectReq] new instance
    pub fn new(select: Vec<(K, Box<dyn EvalEx<Request<K, Q>, EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug, Q> EvalEx<(Request<K, Q>, Option<Link>), EvalResult> for SelectReq<K, Q> {
    //
    //
    fn eval(&self, (req, _): (Request<K, Q>, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectReq", "eval");
        match self.select.get(&req.query_id) {
            Some(eval) => eval.eval(req),
            None => Err(error.err(format!("Request {:?} - is not supported", req.query_id))),
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
unsafe impl<K, Q> Send for SelectReq<K, Q> {}
