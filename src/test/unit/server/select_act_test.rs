#[cfg(test)]

mod select_act {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use sal_sync::services::entity::Cot;
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{domain::{EvalEx, JsonVal, Link}, server::{Query, SelectAct}};
    use super::super::{fake_select_act::{FakeSelectAct1, FakeSelectAct2, FakeSelectAct3}, Command};
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
        let dbg = Dbg::own("select_act.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                json!({ "cot": Cot::Act, "name": "Cmd1", "content": {"data": "Command1 01"} }),
                Ok(json!({ "data": "CmdReply1 01" })),
            ),
            (
                02,
                json!({ "cot": Cot::Act, "name": "Cmd2", "content": {"data": "Command2 02"} }),
                Ok(json!({ "data": "CmdReply2 02" })),
            ),
            (
                03,
                json!({ "cot": Cot::Act, "name": "Cmd3", "content": {"data": "Command3 03"} }),
                Ok(json!({ "data": "CmdReply3 03" })),
            ),
            (
                04,
                json!({ "cot": Cot::Req, "name": "Cmd1", "content": {"data": "Error 04"} }),
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                json!({ "cot": Cot::Req, "name": "Cmd2", "content": {"data": "Error 04"} }),
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                json!({ "cot": Cot::Req, "name": "Cmd3", "content": {"data": "Error 04"} }),
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let select_act = SelectAct::new(vec![
            (Command::Cmd1, Box::new(FakeSelectAct1::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct1", "").err(request));
                }
                let reply = json!({"data": request.replace("Command1", "CmdReply1")});
                Ok(reply)
            }))),
            (Command::Cmd2, Box::new(FakeSelectAct2::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct2", "").err(request));
                }
                let reply = json!({"data": request.replace("Command2", "CmdReply2")});
                Ok(reply)
            }))),
            (Command::Cmd3, Box::new(FakeSelectAct3::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct3", "").err(request));
                }
                let reply = json!({"data": request.replace("Command3", "CmdReply3")});
                Ok(reply)
            }))),
        ]);
        for (step, req, target) in test_data {
            let val: Query<Command> = serde_json::from_value(req).unwrap();
            // MapCtx {
            //     msg_id: step,
            //     map: json!(req).as_object().unwrap().to_owned(),
            // };
            let (loc, rem) = Link::split(&dbg);
            let select_result = select_act.eval((val, Some(rem)));
            let select_target = Ok(None);
            assert!(select_result == select_target, "step {} \nresult: {:?}\ntarget: {:?}", step, select_result, select_target);
            let result: Result<Option<String>, _> = loc.recv_timeout(Duration::from_millis(100));
            match (result, target) {
                (Ok(result), Ok(target)) => {
                    let result: JsonVal = serde_json::from_str(&result.unwrap()).unwrap();
                    assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                }
                (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
                (Err(_), Err(_)) => {}
            }
        }
        test_duration.exit();
    }
}
