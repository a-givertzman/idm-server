use sal_sync::collections::FxIndexMap;
use crate::{domain::{Error, EvalEx, Link}, server::{Content, EvalResult, Event, Request}};
///
/// Matching incoming [Event]s by it's Content
/// - Forwarding matched [Event]s to the associated handlers
pub struct SelectContent {
    select: FxIndexMap<Content, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send>>,
}
//
//
impl SelectContent {
    ///
    /// Returns [SelectContent] new instance
    pub fn new(
        select: Vec<(Content, Box<dyn EvalEx<(Request, Option<Link>), EvalResult> + Send + 'static>)>,
    ) -> Self {
        Self {
            select: FxIndexMap::from_iter(select),
        }
    }
}
//
//
impl EvalEx<(Event, Option<Link>), EvalResult> for SelectContent {
    ///
    /// Selects handler by [Event] content type,
    /// if handler exists, it evaluates with [Request] built from [Event]
    fn eval(&self, (event, link): (Event, Option<Link>)) -> EvalResult {
        let error = Error::new("SelectContent", "eval");
        match self.select.get(&event.content) {
            Some(eval) => match Request::from_event(event) {
                Ok(req) => eval.eval((req, link)),
                Err(err) => Err(error.pass(err)),
            },
            None => Err(error.err(format!("{:?} - is not supported", event.content))),
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
