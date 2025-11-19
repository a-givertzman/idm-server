use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Request}};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectAct<K, Q> {
    select: IndexMap<K, Box<dyn EvalEx<(Request<K, Q>, Option<Link>), EvalResult> + Send>>,
}
//
//
impl<K: std::hash::Hash + std::cmp::Eq, Q> SelectAct<K, Q> {
    ///
    /// Returns [SelectAct] new instance
    pub fn new(select: Vec<(K, Box<dyn EvalEx<(Request<K, Q>, Option<Link>), EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug, Q> EvalEx<(Request<K, Q>, Option<Link>), EvalResult> for SelectAct<K, Q> {
    fn eval(&self, (req, link): (Request<K, Q>, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectAct", "eval");
        match self.select.get(&req.query_id) {
            Some(eval) => {
                let _ = eval.eval((req, link));
                Ok(None)
            },
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
unsafe impl<K, R> Send for SelectAct<K, R> {}
