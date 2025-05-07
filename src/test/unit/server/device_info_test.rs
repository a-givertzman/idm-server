#[cfg(test)]

mod device_info {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use serde::{Deserialize, Serialize};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::{DevId, DeviceInfo}, domain::Eval, server::DeviceInfoRequest};
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
        let dbg = Dbg::own("device_info.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let path = "src/test/unit/server/device_info/";
        let test_data = [
            (
                01,
                111,
                DeviceInfo::new(
                    111,
                    "MAN01".into(),
                    "VEN01".into(),
                    "OC01".into(),
                    "MOD01".into(),
                    "SER01".into(),
                    "NAM01".into(),
                    "DESC01".into(),
                    "W01".into(),
                    "H01".into(),
                    "DEP01".into(),
                    "WEI01".into()
                ),
            ),
            (
                02,
                222,
                DeviceInfo::new(
                    222,
                    "MAN02".into(),
                    "VEN02".into(),
                    "OC02".into(),
                    "MOD02".into(),
                    "SER02".into(),
                    "NAM02".into(),
                    "DESC02".into(),
                    "W02".into(),
                    "H02".into(),
                    "DEP02".into(),
                    "WEI02".into()
                ),
            ),
            (
                03,
                333,
                DeviceInfo::new(
                    333,
                    "MAN03".into(),
                    "VEN03".into(),
                    "OC03".into(),
                    "MOD03".into(),
                    "SER03".into(),
                    "NAM03".into(),
                    "DESC03".into(),
                    "W03".into(),
                    "H03".into(),
                    "DEP03".into(),
                    "WEI03".into()
                ),
            ),
        ];
        let mut dev_info = DeviceInfo::from_path(path);
        for (step, id, target) in test_data {
            let result = dev_info.eval(DevId(id)).unwrap();
            let result: DeviceInfo = serde_json::from_value(result.value).unwrap();
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// Fake Request
    #[derive(Debug, Serialize, Deserialize)]
    struct FakeRequest {
        data: DeviceInfoRequest
    }
}
