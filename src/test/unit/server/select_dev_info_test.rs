#[cfg(test)]

mod select_dev_info {
    use std::{sync::Once, time::Duration};
    use indexmap::IndexMap;
    use sal_core::{dbg::Dbg, error::Error};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{device_info::{DevId, DeviceInfo}, domain::{Eval, JsonVal}, server::{DeviceInfoRequest, JsonCtx, MapCtx, SelectDevInfo}};
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
                    format!("111"),
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
                    format!("222"),
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
                {"devId": 333},
                DeviceInfo::new(
                    format!("333"),
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
                test_data.clone().map(|(_, _, val)| val).into(),
            )
        );
        for (step, msg_id, target) in test_data {
            let req = FakeRequest { data: DeviceInfoRequest { dev_id: format!("{msg_id}") } };
            let val = MapCtx {
                msg_id,
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
        val: IndexMap<String, DeviceInfo>,
    }
    impl FakeDeviceInfo {
        fn new(val: Vec<DeviceInfo>) -> Self {
            Self {
                val: IndexMap::from_iter(val.into_iter().map(|(val)| (val.id.clone(), val))),
            }
        }
    }
    //
    //
    impl Eval<DevId, Result<JsonVal, Error>> for FakeDeviceInfo {
        fn eval(&mut self, id: DevId) -> Result<JsonVal, Error> {
        let error = Error::new("FakeDeviceInfo", "eval");
            match self.val.get(&(id.0)) {
                Some(val) => Ok(json!(val)),
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
