use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Frame}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectAct<K> {
    select: IndexMap<K, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send>>,
}
//
//
impl<K: Hash + Eq> SelectAct<K> {
    ///
    /// Returns [SelectAct] new instance
    pub fn new(select: Vec<(K, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug> EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> for SelectAct<K> {
    //
    //
    fn eval(&self, (frame, link): (Frame<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("SelectAct", "eval");
        match self.select.get(&frame.operation_id) {
            Some(eval) => {
                let _ = eval.eval((frame, link));
                Ok(None)
            },
            None => Err(error.err(format!("Command {:?} - is not supported", frame.operation_id))),
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
unsafe impl<Q> Send for SelectAct<Q> {}
