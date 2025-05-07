#[cfg(test)]

mod class_name {
    use std::{sync::Once, time::{Duration, Instant}};
    use indexmap::IndexMap;
    use sal_core::{dbg::Dbg, error::Error};
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
    fn methos() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("class_name_method");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                111,
                DeviceInfo::new(todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!()),
            ),
            (
                02,
                222,
                DeviceInfo::new(todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!()),
            ),
            (
                03,
                333,
                DeviceInfo::new(todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!()),
            ),
        ];
        let select_dev_info = SelectDevInfo::new(
            FakeDeviceInfo::new(
                test_data.map(|(_, id, val)| (id, val)).into(),
            )
        );
        for (step, id, target) in test_data {
            let req = DeviceInfoRequest { id };
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
}
