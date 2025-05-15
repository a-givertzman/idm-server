#[cfg(test)]

mod dev_stream {
    use std::{sync::Once, time::Duration};
    use sal_core::{dbg::Dbg, error::Error};
    use sal_sync::thread_pool::{self, ThreadPool};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::DevId, domain::{Eval, Link}, server::{DevStream, DevStreamConf, JsonCtx, MapCtx, SelectAct}};
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
        let dbg = Dbg::own("dev_stream.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        // let test_data = [
        // ];
        let thread_pool = ThreadPool::new(&dbg, Some(3));
        let conf = r#"
            devices:
                Dev-01:
                    value: 10.0
                    deviation: 3.0
                    interval: 100   # ms
                Dev-02:
                    value: 20.0
                    deviation: 3.0
                    interval: 100   # ms
                Dev-03:
                    value: 30.0
                    deviation: 3.0
                    interval: 100   # ms
        "#;
        let conf: DevStreamConf = serde_yaml::from_str(conf).unwrap();
        let mut dev_stream = DevStream::new(&dbg, conf.clone(), thread_pool.scheduler());
        for (id, conf) in conf.
        // for (step, req, target) in test_data {
        //     let val = MapCtx {
        //         id: DevId(step.to_string()),
        //         map: json!(req).as_object().unwrap().to_owned(),
        //     };
        //     let (loc, rem) = Link::split(&dbg);
        //     let select_result = dev_stream.eval((val, Some(rem)));
        //     let select_target = Ok(JsonCtx::empty());
        //     assert!(select_result == select_target, "step {} \nresult: {:?}\ntarget: {:?}", step, select_result, select_target);
        //     let result: Result<Option<Reply>, _> = loc.recv_timeout(Duration::from_millis(100));
        //     match (result, target) {
        //         (Ok(result), Ok(target)) => {
        //             let target = Some(Reply { id: step, data: target.to_owned(), error: None });
        //             assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        //         }
        //         (Ok(result), Err(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
        //         (Err(result), Ok(target)) => panic!("step {} \nresult: {:?}\ntarget: {:?}", step, result, target),
        //         (Err(_), Err(_)) => {}
        //     }
        // }
        dev_stream.exit();
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
