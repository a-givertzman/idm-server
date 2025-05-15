use crate::{domain::{Error, Eval, Link}, server::MapCtx};
use super::{Reply, ReqData};

///
/// Fake Req1 handler
pub(crate) struct FakeSelectAct1 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct1 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for FakeSelectAct1 {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("FakeSelectAct1", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: ReqData = data;
                        match (self.ctx)(req.0) {
                            Ok(value) => {
                                link.unwrap().send(Reply { data: value, error: None }).unwrap();
                                Ok(())
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
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
unsafe impl Send for FakeSelectAct1 {}
///
/// Fake Req2 handler
pub(crate) struct FakeSelectAct2 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct2 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for FakeSelectAct2 {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("FakeSelectAct2", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: ReqData = data;
                        match (self.ctx)(req.0) {
                            Ok(value) => {
                                link.unwrap().send(Reply { data: value, error: None }).unwrap();
                                Ok(())
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
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
unsafe impl Send for FakeSelectAct2 {}
///
/// Fake Req3 handler
pub(crate) struct FakeSelectAct3 {
    ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
}
//
//
impl FakeSelectAct3 {
    ///
    /// Returns [SortByX] new instance
    pub fn new(ctx: impl Fn(String) -> Result<String, Error> + Send + 'static) -> Self {
        Self {
            ctx: Box::new(ctx),
        }
    }
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for FakeSelectAct3 {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("FakeSelectAct3", "eval");
        match input.map.get("data") {
            Some(cot) => {
                match serde_json::from_value(cot.to_owned()) {
                    Ok(data) => {
                        let req: ReqData = data;
                        match (self.ctx)(req.0) {
                            Ok(value) => {
                                link.unwrap().send(Reply { data: value, error: None }).unwrap();
                                Ok(())
                            }
                            Err(err) => Err(error.pass(err.to_string())),
                        }
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
unsafe impl Send for FakeSelectAct3 {}
