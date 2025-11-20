use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, QueryId, Request}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectAct {
    select: IndexMap<QueryId, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send>>,
}
//
//
impl SelectAct {
    ///
    /// Returns [SelectAct] new instance
    pub fn new(select: Vec<(QueryId, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl EvalEx<(Request, Option<Link>), EvalResult> for SelectAct {
    //
    //
    fn eval(&self, (req, link): (Request, Option<Link>)) -> EvalResult {
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
unsafe impl Send for SelectAct {}
