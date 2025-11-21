use indexmap::IndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, QueryId, Request}};

///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectReq {
    select: IndexMap<QueryId, Box<dyn EvalEx<Request, EvalResult> + Send>>,
}
//
//
impl SelectReq {
    ///
    /// Returns [SelectReq] new instance
    pub fn new(select: Vec<(QueryId, Box<dyn EvalEx<Request, EvalResult> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select),
        }
    }
}
//
//
impl EvalEx<(Request, Option<Link>), EvalResult> for SelectReq {
    //
    //
    fn eval(&self, (req, _): (Request, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectReq", "eval");
        match self.select.get(&req.query_id) {
            Some(eval) => eval.eval(req),
            None => Err(error.err(format!("Request {:?} - is not supported", req.query_id))),
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
unsafe impl Send for SelectReq {}
