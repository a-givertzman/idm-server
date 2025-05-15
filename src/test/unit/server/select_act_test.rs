#[cfg(test)]

mod select_act {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::DevId, domain::{Eval, Link}, server::{JsonCtx, MapCtx, SelectAct}};
    use super::super::{fake_select_act::{FakeSelectAct1, FakeSelectAct2, FakeSelectAct3}, Command, Reply, ReqData};
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
        let dbg = Dbg::own("select_act.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                FakeRequest { act: Command::Cmd1, data: ReqData("Request1 01".into()) },
                Ok("Reply1 01"),
            ),
            (
                02,
                FakeRequest { act: Command::Cmd2, data: ReqData("Request2 02".into()) },
                Ok("Reply2 02"),
            ),
            (
                03,
                FakeRequest { act: Command::Cmd3, data: ReqData("Request3 03".into()) },
                Ok("Reply3 03"),
            ),
            (
                04,
                FakeRequest { act: Command::Cmd1, data: ReqData("Error 04".into()) },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                FakeRequest { act: Command::Cmd2, data: ReqData("Error 05".into()) },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                FakeRequest { act: Command::Cmd2, data: ReqData("Error 06".into()) },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let mut select_act = SelectAct::new(vec![
            (Command::Cmd1, Box::new(FakeSelectAct1::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct1", "").err(request));
                }
                let reply = request.replace("Request1", "Reply1");
                Ok(reply)
            }))),
            (Command::Cmd2, Box::new(FakeSelectAct2::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct2", "").err(request));
                }
                let reply = request.replace("Request2", "Reply2");
                Ok(reply)
            }))),
            (Command::Cmd3, Box::new(FakeSelectAct3::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct3", "").err(request));
                }
                let reply = request.replace("Request3", "Reply3");
                Ok(reply)
            }))),
        ]);
        for (step, req, target) in test_data {
            let val = MapCtx {
                msg_id: DevId(format!("{step}")),
                map: json!(req).as_object().unwrap().to_owned(),
            };
            let (loc, rem) = Link::split(&dbg);
            let select_result = select_act.eval((val, Some(rem)));
            let select_target = Ok(JsonCtx::empty());
            assert!(select_result == select_target, "step {} \nresult: {:?}\ntarget: {:?}", step, select_result, select_target);
            let result: Result<Option<Reply>, _> = loc.recv_timeout(Duration::from_millis(100));
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    let target = Some(Reply { id: format!("{step}"), data: target.to_owned(), error: None });
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(_), Err(_)) => {}
            }
        }
        test_duration.exit();
    }
    ///
    /// Fake Request
    #[derive(Debug, Serialize, Deserialize)]
    struct FakeRequest {
        act: Command,
        data: ReqData
    }
}
