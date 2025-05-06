use indexmap::IndexMap;
use sal_core::error::Error;
use crate::domain::Eval;
use super::{JsonCtx, MapCtx, Req};
///
/// Matching incoming messages by it's Cot::Req name
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub(crate) struct SelectReq {
    select: IndexMap<Req, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send>>,
}
//
//
impl SelectReq {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(Req, impl Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static)>) -> Self {
        Self {
            select: IndexMap::from_iter(
                select.into_iter().map(|(cot, eval)| -> (Req, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static>) {
                    (cot, Box::new(eval))
                })
            ),
        }
    }
}
//
//
impl Eval<MapCtx, Result<JsonCtx, Error>> for SelectReq {
    fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
        match input.map.get("req") {
            Some(req) => {
                match serde_json::from_value(req.to_owned()) {
                    Ok(req) => {
                        let req: Req = req;
                        match self.select.get_mut(&req) {
                            Some(eval) => {
                                eval.eval(input)
                            },
                            None => todo!(),
                        }
                    }
                    Err(_) => todo!(),
                }
            }
            None => todo!(),
        }
    }
}
//
//
unsafe impl Send for SelectReq {}
