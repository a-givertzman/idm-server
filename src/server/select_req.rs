use std::fmt::Debug;

use indexmap::IndexMap;
use sal_core::error::Error;
use crate::domain::Eval;
use super::{JsonCtx, MapCtx};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectReq<R> {
    select: IndexMap<R, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send>>,
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq> SelectReq<R> {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(R, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select
                // select.into_iter().map(|(cot, eval)| -> (Request, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static>) {
                //     (cot, Box::new(eval))
                // })
            ),
        }
    }
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq + serde::de::DeserializeOwned + Debug> Eval<MapCtx, Result<JsonCtx, Error>> for SelectReq<R> {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        let error = Error::new("SelectReq", "eval");
        match input.map.get("req") {
            Some(req) => {
                match serde_json::from_value(req.to_owned()) {
                    Ok(req) => {
                        let req: R = req;
                        match self.select.get_mut(&req) {
                            Some(eval) => {
                                eval.eval(input)
                            },
                            None => Err(error.err(format!("Request {:?} - is not supported", req))),
                        }
                    }
                    Err(err) => Err(error.pass_with(format!("Request can't be parsed {:#?}", req), err.to_string())),
                }
            }
            None => Err(error.err(format!("Field 'req' missed in the request {:#?}", input.map))),
        }
    }
}
//
//
unsafe impl<R> Send for SelectReq<R> {}
