#[cfg(test)]

mod select_req {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{domain::Eval, server::{MapCtx, SelectReq}, test::unit::server::fake_select_req::{FakeSelectReq1, FakeSelectReq2, FakeSelectReq3}};
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
    fn eval() {
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
                json!({ "req": "Req1", "data": "Request1 01" }),
                Ok(json!({ "data": "Reply1 01" })),
            ),
            (
                02,
                json!({ "req": "Req2", "data": "Request2 02" }),
                Ok(json!({ "data": "Reply2 02" })),
            ),
            (
                03,
                json!({ "req": "Req3", "data": "Request3 03" }),
                Ok(json!({ "data": "Reply3 03" })),
            ),
            (
                04,
                json!({ "req": "Req1", "data": "Error 04" }),
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                json!({ "req": "Req2", "data": "Error 05" }),
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                json!({ "req": "Req2", "data": "Error 06" }),
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let mut select_req = SelectReq::new(vec![
            (Request::Req1, Box::new(FakeSelectReq1::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                let reply = json!({"data": request.replace("Request1", "Reply1")});
                Ok(reply)
            }))),
            (Request::Req2, Box::new(FakeSelectReq2::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                let reply = json!({"data": request.replace("Request2", "Reply2")});
                Ok(reply)
            }))),
            (Request::Req3, Box::new(FakeSelectReq3::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                let reply = json!({"data": request.replace("Request3", "Reply3")});
                Ok(reply)
            }))),
        ]);
        for (step, req, target) in test_data {
            let val = MapCtx {
                msg_id: step,
                map: req.as_object().unwrap().to_owned(),
            };
            let result = select_req.eval((val, None));
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    let result = result.value;
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
    /// Fake List of API requiests
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    enum Request {
        Req1,
        Req2,
        Req3,
    }
}
