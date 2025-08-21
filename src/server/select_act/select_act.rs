use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Query}};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectAct<R> {
    select: IndexMap<R, Box<dyn EvalEx<(Query<R>, Option<Link>), EvalResult> + Send>>,
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq> SelectAct<R> {
    ///
    /// Returns [SelectAct] new instance
    pub fn new(select: Vec<(R, Box<dyn EvalEx<(Query<R>, Option<Link>), EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select
            ),
        }
    }
}
//
//
impl<R: Borrow<R> + Hash + Eq + serde::de::DeserializeOwned + Debug> EvalEx<(Query<R>, Option<Link>), EvalResult> for SelectAct<R> {
    fn eval(&self, (query, link): (Query<R>, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectAct", "eval");
        match self.select.get(&query.name) {
            Some(eval) => {
                let _ = eval.eval((query, link));
                Ok(None)
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
unsafe impl<R> Send for SelectAct<R> {}
