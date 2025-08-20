use indexmap::IndexMap;
use crate::domain::{Error, Eval, JsonVal, Link};
use super::{BytesCtx, Cot, JsonCtx, MapCtx};
///
/// Matching incoming messages by it's Cot
/// - Forwarding matched messages to the associated handlers
/// - Returns bytes and id of messages to be sent over TCP
pub(crate) struct SelectCot {
    select: IndexMap<Cot, Box<dyn Eval<(MapCtx, Option<Link>), Result<JsonCtx, Error>> + Send>>,
}
//
//
impl SelectCot {
    ///
    /// Returns [SortByX] new instance
    pub fn new(select: Vec<(Cot, Box<dyn Eval<(MapCtx, Option<Link>), Result<JsonCtx, Error>> + Send + 'static>)>) -> Self {
        Self {
            select: IndexMap::from_iter(select
                // select.into_iter().map(|(cot, eval)| -> (Cot, Box<dyn Eval<MapCtx, Result<JsonCtx, Error>> + Send + 'static>) {
                //     (cot, Box::new(eval))
                // })
            ),
        }
    }
}
//
//
impl Eval<(BytesCtx, Option<Link>), Result<JsonCtx, Error>> for SelectCot {
    fn eval(&mut self, (input, link): (BytesCtx, Option<Link>)) -> Result<JsonCtx, Error> {
        let error = Error::new("SelectCot", "eval");
        match serde_json::from_slice(&input.bytes) {
            Ok(value) => {
                let value: JsonVal = value;
                match value.as_object() {
                    Some(map) => {
                        match Cot::try_from_map(map, "cot") {
                            Ok(cot) => {
                                match self.select.get_mut(&cot) {
                                    Some(eval) => {
                                        match cot {
                                            Cot::Act => eval.eval((MapCtx { map: map.to_owned(), msg_id: input.msg_id }, link)),
                                            Cot::Req => eval.eval((MapCtx { map: map.to_owned(), msg_id: input.msg_id }, None)),
                                            _ => Err(error.err(format!("Cot {:?} - is not supported", cot))),
                                        }
                                    },
                                    None => Err(error.err(format!("Cot {:?} - is not supported", cot))),
                                }
                            }
                            Err(err) => Err(error.pass_with("Parse cot error", err)),
                        }
                    }
                    None => Err(error.err(format!("Wrong request format, map expected, but found {:#?}", value))),
                }
            }
            Err(err) => {
                match std::str::from_utf8(&input.bytes) {
                    Ok(req) => Err(error.pass_with(format!("Request can't be parsed from {:#?}", req), err.to_string())),
                    Err(err) => Err(
                        error.pass_with(
                            format!("Request can't be parsed from bytes wich can't be converted into string, bytes:\n\t{:?}", input.bytes),
                            err.to_string(),
                        ),
                    )
                }
            }
        }
    }
}
