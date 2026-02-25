#[cfg(test)]

mod select_cot {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{domain::{EvalEx, JsonVal, Link}, server::{Content, Cot, Frame, Query, Reply, SelectAct, SelectCot, SelectReq, extract}};
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
                Frame { id: 0, operation_id: Req::Req1, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Request1 01\"}".as_bytes().into() },
                Ok("Reply1 01"),
            ),
            (
                02,
                Cot::Req,
                Frame { id: 0, operation_id: Req::Req2, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Request2 02\"}".as_bytes().into() },
                Ok("Reply2 02"),
            ),
            (
                03,
                Cot::Req,
                Frame { id: 0, operation_id: Req::Req3, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Request3 03\"}".as_bytes().into() },
                Ok("Reply3 03"),
            ),
            (
                04,
                Cot::Req,
                Frame { id: 0, operation_id: Req::Req1, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Error 04\"}".as_bytes().into() },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                Cot::Req,
                Frame { id: 0, operation_id: Req::Req2, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Error 05\"}".as_bytes().into() },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                Cot::Req,
                Frame { id: 0, operation_id: Req::Req3, cot: Cot::Req, content: Content::Json, bytes: "{\"TestString\":\"Error 06\"}".as_bytes().into() },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
            (
                07,
                Cot::Act,
                Frame { id: 0, operation_id: Req::Cmd1, cot: Cot::Act, content: Content::Json, bytes: "{\"TestString\":\"Command1 07\"}".as_bytes().into() },
                Ok("CmdReply1 07"),
            ),
            (
                08,
                Cot::Act,
                Frame { id: 0, operation_id: Req::Cmd2, cot: Cot::Act, content: Content::Json, bytes: "{\"TestString\":\"Command2 08\"}".as_bytes().into() },
                Ok("CmdReply2 08"),
            ),
            (
                09,
                Cot::Act,
                Frame { id: 0, operation_id: Req::Cmd3, cot: Cot::Act, content: Content::Json, bytes: "{\"TestString\":\"Command3 09\"}".as_bytes().into() },
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
