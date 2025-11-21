#[cfg(test)]

mod select_act {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{domain::{EvalEx, JsonVal, Link}, server::{Content, Cot, Query, Request, SelectAct}};
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
                Request { event_id: 0, query_id: Command::Cmd1, cot: Cot::Act, content: Content::Json, query: Query::TestString(format!("Command1 01")) },
                Ok(format!("CmdReply1 01")),
            ),
            (
                02,
                Request { event_id: 0, query_id: Command::Cmd2, cot: Cot::Act, content: Content::Json, query: Query::TestString(format!("Command2 02")) },
                Ok(format!("CmdReply2 02")),
            ),
            (
                03,
                Request { event_id: 0, query_id: Command::Cmd3, cot: Cot::Act, content: Content::Json, query: Query::TestString(format!("Command3 03")) },
                Ok(format!("CmdReply3 03")),
            ),
            (
                04,
                Request { event_id: 0, query_id: Command::Cmd1, cot: Cot::Req, content: Content::Json, query: Query::TestString(format!("Error 04")) },
                Err(Error::new("", &dbg).err("Error 04")),
            ),
            (
                05,
                Request { event_id: 0, query_id: Command::Cmd2, cot: Cot::Req, content: Content::Json, query: Query::TestString(format!("Error 05")) },
                Err(Error::new("", &dbg).err("Error 05")),
            ),
            (
                06,
                Request { event_id: 0, query_id: Command::Cmd3, cot: Cot::Req, content: Content::Json, query: Query::TestString(format!("Error 06")) },
                Err(Error::new("", &dbg).err("Error 06")),
            ),
        ];
        let select_act = SelectAct::new(vec![
            (Command::Cmd1, Box::new(FakeSelectAct1::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct1", "").err(request));
                }
                Ok(request.replace("Command1", "CmdReply1"))
            }))),
            (Command::Cmd2, Box::new(FakeSelectAct2::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct2", "").err(request));
                }
                Ok(request.replace("Command2", "CmdReply2"))
            }))),
            (Command::Cmd3, Box::new(FakeSelectAct3::new(|request| {
                if request.to_lowercase().contains("error") {
                    return Err(Error::new("FakeSelectAct3", "").err(request));
                }
                Ok(request.replace("Command3", "CmdReply3"))
            }))),
        ]);
        for (step, req, target) in test_data {
            let (loc, rem) = Link::split(&dbg);
            let select_result = select_act.eval((req, Some(rem)));
            let select_target: Result<Option<_>, _> = Ok(None);
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
