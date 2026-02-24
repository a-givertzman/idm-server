use std::{borrow::Borrow, fmt::Debug, hash::Hash};

use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{Cot, EvalResult, Frame}};
///
/// Matching incoming messages by it's Cot
/// - Forwarding matched messages to the associated handlers
pub struct SelectCot<K> {
    select: IndexMap<Cot, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send>>,
}
//
//
impl<K> SelectCot<K> {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(Cot, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug> EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> for SelectCot<K> {
    //
    //
    fn eval(&self, (query, link): (Frame<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("SelectCot", "eval");
        match self.select.get(&query.cot) {
            Some(eval) => {
                match query.cot {
                    Cot::Act => eval.eval((query, link)),
                    Cot::Req => eval.eval((query, None)),
                    _ => Err(error.err(format!("Cot {:?} - is not supported", query.cot))),
                }
            },
            None => Err(error.err(format!("Cot {:?} - is not supported", query.cot))),
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
