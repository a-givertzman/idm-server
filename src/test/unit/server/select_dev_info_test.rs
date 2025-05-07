#[cfg(test)]

mod select_dev_info {
    use std::{sync::Once, time::Duration};
    use indexmap::IndexMap;
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::{DevId, DeviceInfo}, domain::Eval, server::{DeviceInfoRequest, JsonCtx, MapCtx, SelectDevInfo}};
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
        let dbg = Dbg::own("select_dev_info.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                111,
                DeviceInfo::new(
                    01,
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
                    01,
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
                    01,
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
        let mut select_dev_info = SelectDevInfo::new(
            FakeDeviceInfo::new(
                test_data.clone().map(|(_, id, val)| (id, val)).into(),
            )
        );
        for (step, id, target) in test_data {
            let req = FakeRequest { data: DeviceInfoRequest { id } };
            let val = MapCtx {
                id: DevId(id),
                map: json!(req).as_object().unwrap().to_owned(),
            };
            let result = select_dev_info.eval(val).unwrap();
            let result: DeviceInfo = serde_json::from_value(result.value).unwrap();
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// Fake DeviceInfo for testing only
    pub struct FakeDeviceInfo {
        val: IndexMap<u32, DeviceInfo>,
    }
    impl FakeDeviceInfo {
        fn new(val: Vec<(u32, DeviceInfo)>) -> Self {
            Self {
                val: IndexMap::from_iter(val.into_iter().map(|(id, val)| (id, val))),
            }
        }
    }
    //
    //
    impl Eval<DevId, Result<JsonCtx, Error>> for FakeDeviceInfo {
        fn eval(&mut self, id: DevId) -> Result<JsonCtx, Error> {
        let error = Error::new("FakeDeviceInfo", "eval");
            match self.val.get(&(id.0)) {
                Some(val) => Ok(JsonCtx { id, value: json!(val) }),
                None => Err(error.err(format!("id {} - is not found in the test_data", id.0))),
            }
        }
    }
    ///
    /// Fake Request
    #[derive(Debug, Serialize, Deserialize)]
    struct FakeRequest {
        data: DeviceInfoRequest
    }
}
