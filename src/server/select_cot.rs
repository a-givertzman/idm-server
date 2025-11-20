use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{Cot, EvalResult, Request}};
///
/// Matching incoming messages by it's Cot
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectCot {
    select: IndexMap<Cot, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send>>,
}
//
//
impl SelectCot {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(Cot, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl EvalEx<(Request, Option<Link>), EvalResult> for SelectCot {
    //
    //
    fn eval(&self, (query, link): (Request, Option<Link>)) -> EvalResult {
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
    //
    //
    fn exit(&self) {
        for (_, e) in &self.select {
            e.exit();
        }
    }
}
