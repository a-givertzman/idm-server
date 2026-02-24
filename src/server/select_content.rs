use std::{borrow::Borrow, fmt::Debug, hash::Hash};
use sal_sync::collections::FxIndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{Content, EvalResult, Frame}};
///
/// Matching incoming [Event]s by it's Content
/// - Forwarding matched [Event]s to the associated handlers
pub struct SelectContent<K> {
    select: FxIndexMap<Content, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send>>,
}
//
//
impl<K> SelectContent<K> {
    ///
    /// Returns [SelectContent] new instance
    pub fn new(
        select: Vec<(Content, Box<dyn EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> + Send + 'static>)>,
    ) -> Self {
        Self {
            select: FxIndexMap::from_iter(select),
        }
    }
}
//
//
impl<K: Borrow<K> + Hash + Eq + Debug + Copy> EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> for SelectContent<K> {
    ///
    /// Selects handler by [Frame] content type,
    /// if handler exists, it evaluates with [Request] built from [Frame]
    fn eval(&self, (frame, link): (Frame<K>, Option<Link>)) -> EvalResult<K> {
        let error = Error::new("SelectContent", "eval");
        match self.select.get(&frame.content) {
            Some(eval) => eval.eval((frame, link)),
            None => Err(error.err(format!("{:?} - is not supported", frame.content))),
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
