#[cfg(test)]

mod select_req {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::DevId, domain::Eval, server::{JsonCtx, MapCtx, SelectReq}};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing such functionality / behavior
    #[test]
    fn methos() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("select_req.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                FakeRequest { req: Request::Req1, data: ReqData("Request 01".into()) },
                Ok("Reply1 01"),
            ),
            (
                02,
                FakeRequest { req: Request::Req2, data: ReqData("Request 02".into()) },
                Ok("Reply2 02"),
            ),
            (
                03,
                FakeRequest { req: Request::Req3, data: ReqData("Request 03".into()) },
                Ok("Reply3 03"),
            ),
            (
                04,
                FakeRequest { req: Request::Req1, data: ReqData("Error 04".into()) },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                FakeRequest { req: Request::Req2, data: ReqData("Error 05".into()) },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                FakeRequest { req: Request::Req2, data: ReqData("Error 06".into()) },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let mut select_req = SelectReq::new(
            vec![
                (Request::Req1, Box::new(FakeSelectReq1::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request", "Reply1");
                    Ok(reply)
                }))),
                (Request::Req2, Box::new(FakeSelectReq2::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request", "Reply2");
                    Ok(reply)
                }))),
                (Request::Req3, Box::new(FakeSelectReq3::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request", "Reply3");
                    Ok(reply)
                }))),
            ]
        );
        for (step, req, target) in test_data {
            let val = MapCtx {
                id: DevId(step),
                map: json!(req).as_object().unwrap().to_owned(),
            };
            let result = select_req.eval(val);
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    let target = JsonCtx { id: DevId(step), value: json!(target) };
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(_), Err(_)) => {}
            }
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// Request kind 1
    #[derive(Debug, Serialize, Deserialize)]
    struct ReqData(pub String);
    ///
    /// Fake Request
    #[derive(Debug, Serialize, Deserialize)]
    struct FakeRequest {
        req: Request,
        data: ReqData
    }
    ///
    /// Fake List of API requiests
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    enum Request {
        Req1,
        Req2,
        Req3,
    }
    ///
    /// Reply
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    struct Reply {
        data: String,
        error: Option<String>,
    }
    ///
    /// Fake Req1 handler
    pub(crate) struct FakeSelectReq1 {
        ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
    }
    //
    //
    impl FakeSelectReq1 {
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
    impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq1 {
        fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
            let error = Error::new("FakeSelectReq1", "eval");
            match input.map.get("data") {
                Some(cot) => {
                    match serde_json::from_value(cot.to_owned()) {
                        Ok(data) => {
                            let req: ReqData = data;
                            match (self.ctx)(req.0) {
                                Ok(value) => Ok(JsonCtx { id: input.id, value: json!(value) }),
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
    unsafe impl Send for FakeSelectReq1 {}
    ///
    /// Fake Req2 handler
    pub(crate) struct FakeSelectReq2 {
        ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
    }
    //
    //
    impl FakeSelectReq2 {
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
    impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq2 {
        fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
            let error = Error::new("FakeSelectReq2", "eval");
            match input.map.get("data") {
                Some(cot) => {
                    match serde_json::from_value(cot.to_owned()) {
                        Ok(data) => {
                            let req: ReqData = data;
                            match (self.ctx)(req.0) {
                                Ok(value) => Ok(JsonCtx { id: input.id, value: json!(value) }),
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
    unsafe impl Send for FakeSelectReq2 {}
    ///
    /// Fake Req3 handler
    pub(crate) struct FakeSelectReq3 {
        ctx: Box<dyn Fn(String) -> Result<String, Error> + Send>,
    }
    //
    //
    impl FakeSelectReq3 {
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
    impl Eval<MapCtx, Result<JsonCtx, Error>> for FakeSelectReq3 {
        fn eval(&mut self, input: MapCtx) -> Result<JsonCtx, Error> {
            let error = Error::new("FakeSelectReq3", "eval");
            match input.map.get("data") {
                Some(cot) => {
                    match serde_json::from_value(cot.to_owned()) {
                        Ok(data) => {
                            let req: ReqData = data;
                            match (self.ctx)(req.0) {
                                Ok(value) => Ok(JsonCtx { id: input.id, value: json!(value) }),
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
    unsafe impl Send for FakeSelectReq3 {}
}
