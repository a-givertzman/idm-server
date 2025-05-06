use indexmap::IndexMap;
use sal_core::error::Error;
use crate::domain::Eval;

use super::{BytesCtx, Cot, JsonCtx, MapCtx};
///
/// Matching incoming messages by it's Cot
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub(crate) struct SelectCot {
    select: IndexMap<Cot, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send>>,
}
//
//
impl SelectCot {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(Cot, impl Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static)>) -> Self {
        Self {
            select: IndexMap::from_iter(
                select.into_iter().map(|(cot, eval)| -> (Cot, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static>) {
                    (cot, Box::new(eval))
                })
            ),
        }
    }
}
//
//
impl Eval<BytesCtx, Result<JsonCtx, Error>> for SelectCot {
    fn eval(&mut self, input: BytesCtx) -> Result<JsonCtx, Error> {
        match serde_json::from_slice(&input.bytes) {
            Ok(value) => {
                let value: serde_json::Value = value;
                match value.as_object() {
                    Some(map) => {
                        match map.get("cot") {
                            Some(cot) => {
                                match serde_json::from_value(cot.to_owned()) {
                                    Ok(cot) => {
                                        let cot: Cot = cot;
                                        match self.select.get_mut(&cot) {
                                            Some(eval) => {
                                                eval.eval(MapCtx { map: map.to_owned(), id: input.id })
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
                    None => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }
}
