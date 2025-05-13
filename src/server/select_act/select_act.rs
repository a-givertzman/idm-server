use std::fmt::Debug;
use indexmap::IndexMap;
use sal_core::error::Error;
use crate::{domain::{Eval, Link}, server::{JsonCtx, MapCtx}};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub struct SelectAct<R> {
    select: IndexMap<R, Box<dyn Eval<(MapCtx, Option<Link>), Result<(), Error>> + Send>>,
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq> SelectAct<R> {
    ///
    /// Returns [SelectAct] new instance
    pub fn new(select: Vec<(R, Box<dyn Eval<(MapCtx, Option<Link>), Result<(), Error>> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select
            ),
        }
    }
}
//
//
impl<R: std::hash::Hash + std::cmp::Eq + serde::de::DeserializeOwned + Debug> Eval<(MapCtx, Option<Link>), Result<JsonCtx, Error>> for SelectAct<R> {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<JsonCtx, Error> {
        let error = Error::new("SelectAct", "eval");
        match input.map.get("act") {
            Some(req) => {
                match serde_json::from_value(req.to_owned()) {
                    Ok(req) => {
                        let req: R = req;
                        match self.select.get_mut(&req) {
                            Some(eval) => {
                                let _ = eval.eval((input, link));
                                Ok(JsonCtx::empty())
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
unsafe impl<R> Send for SelectAct<R> {}
