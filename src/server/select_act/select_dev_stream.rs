use sal_core::error::Error;

use crate::{domain::{Eval, Link}, server::MapCtx};

pub struct SelectDevStream {}
//
//
impl SelectDevStream {
    ///
    /// Returns [SelectDevStream] new instance
    pub fn new(
        // ctx: impl Eval<DevId, Result<JsonCtx, Error>> + Send + 'static
    ) -> Self {
        Self {
            // ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for SelectDevStream {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("SelectDevDoc", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: String = data;
                        // match self.ctx.eval(DevId(req.id)) {
                        //     Ok(value) => Ok(value),
                        //     Err(err) => Err(error.pass(err.to_string())),
                        // }
                        Err(error.err("Not implemented"))
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            None => Err(error.err(format!("data field is not found in {:#?}", input.map))),
        }
    }
}
//
//
unsafe impl Send for SelectDevStream {}
