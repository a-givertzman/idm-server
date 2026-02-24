use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use sal_sync::collections::FxIndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Frame}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectReq<K> {
    select: FxIndexMap<K, Box<dyn EvalEx<Frame<K>, EvalResult<K>> + Send>>,
}
//
//
impl<K: Hash + Eq> SelectReq<K> {
    ///
    /// Returns [SelectReq] new instance
    pub fn new(select: Vec<(K, Box<dyn EvalEx<Frame<K>, EvalResult<K>> + Send + 'static>)>) -> Self {
        Self {
            select: FxIndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug> EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> for SelectReq<K> {
    //
    //
    fn eval(&self, (req, _): (Frame<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("SelectReq", "eval");
        match self.select.get(&req.operation_id) {
            Some(eval) => eval.eval(req),
            None => Err(error.err(format!("Request {:?} - is not supported", req.operation_id))),
        }
    }
    ///
    /// Halts all configured hanblers
    fn exit(&self) {
        for (_, sel) in &self.select {
            sel.exit();
        }
    }
}
//
//
unsafe impl<K> Send for SelectReq<K> {}
