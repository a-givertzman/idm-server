#[cfg(test)]

mod select_req {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{domain::EvalEx, server::{Content, Cot, Query, Reply, Request, SelectReq, extract}, test::unit::server::fake_select_req::{FakeSelectReq1, FakeSelectReq2, FakeSelectReq3}};
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
        DebugSession::new().filter(LogLevel::Debug).init();
        init_once();
        init_each();
        let dbg = Dbg::own("select_req.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                Request { event_id: 0, query_id: Req::Req1, cot: Cot::Req, content: Content::Json, query: Query::TestString("Request1 01".into()) },
                Ok(format!("Reply1 01")),
            ),
            (
                02,
                Request { event_id: 0, query_id: Req::Req2, cot: Cot::Req, content: Content::Json, query: Query::TestString("Request2 02".into()) },
                Ok(format!("Reply2 02")),
            ),
            (
                03,
                Request { event_id: 0, query_id: Req::Req3, cot: Cot::Req, content: Content::Json, query: Query::TestString("Request3 03".into()) },
                Ok(format!("Reply3 03")),
            ),
            (
                04,
                Request { event_id: 0, query_id: Req::Req1, cot: Cot::Req, content: Content::Json, query: Query::TestString("Error 04".into()) },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                Request { event_id: 0, query_id: Req::Req2, cot: Cot::Req, content: Content::Json, query: Query::TestString("Error 05".into()) },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                Request { event_id: 0, query_id: Req::Req3, cot: Cot::Req, content: Content::Json, query: Query::TestString("Error 06".into()) },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let select_req = SelectReq::new(vec![
            (Req::Req1, Box::new(FakeSelectReq1::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                Ok(request.replace("Request1", "Reply1"))
            }))),
            (Req::Req2, Box::new(FakeSelectReq2::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                Ok(request.replace("Request2", "Reply2"))
            }))),
            (Req::Req3, Box::new(FakeSelectReq3::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectReq2", "").err(request));
                }
                Ok(request.replace("Request3", "Reply3"))
            }))),
        ]);
        for (step, req, target) in test_data {
            let result = select_req.eval((req, None));
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    let result = extract!(result.unwrap().reply, Reply::TestString).unwrap();
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
    enum Req {
        Req1,
        Req2,
        Req3,
    }
}
