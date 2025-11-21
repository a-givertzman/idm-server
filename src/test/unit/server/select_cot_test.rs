#[cfg(test)]

mod select_cot {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{domain::{EvalEx, JsonVal, Link}, server::{Content, Cot, Query, Reply, Request, SelectAct, SelectCot, SelectReq, extract}};
    use super::super::{fake_select_act::{FakeSelectAct1, FakeSelectAct2, FakeSelectAct3}, fake_select_req::{FakeSelectReq1, FakeSelectReq2, FakeSelectReq3}};
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
        let dbg = Dbg::own("select_cot.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Req1, cot: Cot::Req, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req1", "content": {"data": "Request1 01"} }),
                Ok("Reply1 01"),
            ),
            (
                02,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Req2, cot: Cot::Req, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req2", "content": {"data": "Request2 02"} }),
                Ok("Reply2 02"),
            ),
            (
                03,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Req3, cot: Cot::Req, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req3", "content": {"data": "Request3 03"} }),
                Ok("Reply3 03"),
            ),
            (
                04,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req1", "content": {"data": "Error 04"} }),
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req2", "content": {"data": "Error 05"} }),
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                Cot::Req,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Req, "name": "Req3", "content": {"data": "Error 06"} }),
                Err(Error::new("", &dbg).err("Error 06")),
            ),
            (
                07,
                Cot::Act,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Act, "name": "Cmd1", "content": {"data": "Command1 07"} }),
                Ok("CmdReply1 07"),
            ),
            (
                08,
                Cot::Act,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Act, "name": "Cmd2", "content": {"data": "Command2 08"} }),
                Ok("CmdReply2 08"),
            ),
            (
                09,
                Cot::Act,
                Request { event_id: 0, query_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(serde_json::to_string(&json!({"data": "Command1 01"})).unwrap()) },
                // json!({ "cot": Cot::Act, "name": "Cmd3", "content": {"data": "Command3 09"} }),
                Ok("CmdReply3 09"),
            ),
        ];
        let select = SelectCot::new(vec![
            (Cot::Act, Box::new(SelectAct::new(vec![
                (Req::Cmd1, Box::new(FakeSelectAct1::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct1", "").err(request));
                    }
                    Ok(request.replace("Command1", "CmdReply1"))
                }))),
                (Req::Cmd2, Box::new(FakeSelectAct2::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct2", "").err(request));
                    }
                    Ok(request.replace("Command2", "CmdReply2"))
                }))),
                (Req::Cmd3, Box::new(FakeSelectAct3::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct3", "").err(request));
                    }
                    Ok(request.replace("Command3", "CmdReply3"))
                }))),
            ]))),
            (Cot::Req, Box::new(SelectReq::new(vec![
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
            ]))),
        ]);
        for (step, req_cot, req, target) in test_data {
            match req_cot {
                Cot::Act => {
                    let (loc, rem) = Link::split(&dbg);
                    let select_result = select.eval((req, Some(rem)));
                    let select_target = Ok(None);
                    assert!(select_result == select_target, "step {} \nresult: {:?}\ntarget: {:?}", step, select_result, select_target);
                    let result: Result<Option<String>, _> = loc.recv_timeout(Duration::from_millis(100));
                    match (result, target) {
                        (Ok(result), Ok(target)) => {
                            let result: JsonVal = serde_json::from_str(&result.unwrap()).unwrap();
                            assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
                        }
                        (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(_), Err(_)) => {}
                    }
                }
                Cot::Req => {
                    let result = select.eval((req, None));
                    match (result, target) {
                        (Ok(result), Ok(target)) => {
                            let result = extract!(result.unwrap().reply, Reply::TestString).unwrap();
                            assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
                        }
                        (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(_), Err(_)) => {}
                    }
                }
                _ => panic!("step {step} Unexpected cot {:?} in the test_data", req_cot)
            }
            
        }
        // assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// Fake List of API requiests
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    enum Req {
        Req1,
        Req2,
        Req3,
        Cmd1,
        Cmd2,
        Cmd3,
        None,
    }
}
