#[cfg(test)]

mod select_cot {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::DevId, domain::{Eval, Link}, server::{BytesCtx, Cot, JsonCtx, SelectAct, SelectCot, SelectReq}};
    use super::super::{Reply, fake_select_act::{FakeSelectAct1, FakeSelectAct2, FakeSelectAct3}, fake_select_req::{FakeSelectReq1, FakeSelectReq2, FakeSelectReq3}, ReqData};
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
        let dbg = Dbg::own("select_cot.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                FakeRequest { cot: Cot::Req, req: Request::Req1, data: ReqData("Request1 01".into()), ..Default::default() },
                Ok("Reply1 01"),
            ),
            (
                02,
                FakeRequest { cot: Cot::Req, req: Request::Req2, data: ReqData("Request2 02".into()), ..Default::default() },
                Ok("Reply2 02"),
            ),
            (
                03,
                FakeRequest { cot: Cot::Req, req: Request::Req3, data: ReqData("Request3 03".into()), ..Default::default() },
                Ok("Reply3 03"),
            ),
            (
                04,
                FakeRequest { cot: Cot::Req, req: Request::Req1, data: ReqData("Error 04".into()), ..Default::default() },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                FakeRequest { cot: Cot::Req, req: Request::Req2, data: ReqData("Error 05".into()), ..Default::default() },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                FakeRequest { cot: Cot::Req, req: Request::Req2, data: ReqData("Error 06".into()), ..Default::default() },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
            (
                07,
                FakeRequest { cot: Cot::Act, act: Command::Cmd1, data: ReqData("Command1 07".into()), ..Default::default() },
                Ok("CmdReply1 07"),
            ),
            (
                08,
                FakeRequest { cot: Cot::Act, act: Command::Cmd2, data: ReqData("Command2 08".into()), ..Default::default() },
                Ok("CmdReply2 08"),
            ),
            (
                09,
                FakeRequest { cot: Cot::Act, act: Command::Cmd3, data: ReqData("Command3 09".into()), ..Default::default() },
                Ok("CmdReply3 09"),
            ),
        ];
        let mut select = SelectCot::new(vec![
            (Cot::Act, Box::new(SelectAct::new(vec![
                (Command::Cmd1, Box::new(FakeSelectAct1::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct1", "").err(request));
                    }
                    let reply = request.replace("Command1", "CmdReply1");
                    Ok(reply)
                }))),
                (Command::Cmd2, Box::new(FakeSelectAct2::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct2", "").err(request));
                    }
                    let reply = request.replace("Command2", "CmdReply2");
                    Ok(reply)
                }))),
                (Command::Cmd3, Box::new(FakeSelectAct3::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectAct3", "").err(request));
                    }
                    let reply = request.replace("Command3", "CmdReply3");
                    Ok(reply)
                }))),
            ]))),
            (Cot::Req, Box::new(SelectReq::new(vec![
                (Request::Req1, Box::new(FakeSelectReq1::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request1", "Reply1");
                    Ok(reply)
                }))),
                (Request::Req2, Box::new(FakeSelectReq2::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request2", "Reply2");
                    Ok(reply)
                }))),
                (Request::Req3, Box::new(FakeSelectReq3::new(|request| {
                    if request.to_lowercase().contains("error") {
                        return Err(Error::new("FakeSelectReq2", "").err(request));
                    }
                    let reply = request.replace("Request3", "Reply3");
                    Ok(reply)
                }))),
            ]))),
        ]);
        for (step, req, target) in test_data {
            let bytes = serde_json::to_vec(&req).unwrap();
            let val = BytesCtx {
                id: DevId(format!("{step}")),
                bytes,
            };
            match req.cot {
                Cot::Act => {
                    let (loc, rem) = Link::split(&dbg);
                    let select_result = select.eval((val, Some(rem)));
                    let select_target = Ok(JsonCtx::empty());
                    assert!(select_result == select_target, "step {} \nresult: {:?}\ntarget: {:?}", step, select_result, select_target);
                    let result: Result<Option<Reply>, _> = loc.recv_timeout(Duration::from_millis(100));
                    match (result, target) {
                        (Ok(result), Ok(target)) => {
                            let target = Some(Reply { id: format!("{step}"), data: target.to_owned(), error: None });
                            assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
                        }
                        (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(_), Err(_)) => {}
                    }
                }
                Cot::Req => {
                    let result = select.eval((val, None));
                    match (result, target) {
                        (Ok(result), Ok(target)) => {
                            let target = JsonCtx::new(DevId(format!("{step}")), json!(target));
                            assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
                        }
                        (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                        (Err(_), Err(_)) => {}
                    }
                }
                _ => panic!("step {step} Unexpected cot {:?}", req.cot)
            }
            
        }
        // assert!(result == target, "step {step} \nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// Fake Request
    #[derive(Debug, Serialize, Deserialize)]
    struct FakeRequest {
        cot: Cot,
        act: Command,
        req: Request,
        data: ReqData
    }
    impl Default for FakeRequest {
        fn default() -> Self {
        Self {
            cot: Cot::Inf,
            act: Command::None,
            req: Request::None,
            data: ReqData(String::new()),
        }
    }
    }
    ///
    /// Fake List of API requiests
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    enum Command {
        Cmd1,
        Cmd2,
        Cmd3,
        None,
    }
    ///
    /// Fake List of API requiests
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Hash)]
    enum Request {
        Req1,
        Req2,
        Req3,
        None,
    }
}
