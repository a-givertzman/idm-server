#[cfg(test)]

mod select_dev_info {
    use std::{sync::Once, time::Duration};
    use indexmap::IndexMap;
    use sal_core::{dbg::Dbg, error::Error};
    use sal_sync::services::entity::Cot;
    use serde_json::json;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
    use crate::{device::DevId, domain::{EvalEx, JsonVal}, server::{EvalResult, Query, Request, SelectDevInfo}};
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
        let dbg = Dbg::own("select_dev_info.eval");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                01,
                "Device-111",
                json!({"dev-id": "Device-111"}),
                json!({
                    "cot": "req",
                    "data": {
                        "id": "111",
                        "manufacturer": "MAN01",
                        "vendor": "VEN01",
                        "order-code": "OC01",
                        "model": "MOD01",
                        "serial": "SER01",
                        "name": "NAM01",
                        "description": "DESC01",
                        "width": "W01",
                        "height": "H01",
                        "depth": "DEP01",
                        "weight": "WEI01"
                    },
                }),
            ),
            (
                02,
                "Device-222",
                json!({"dev-id": "Device-222"}),
                json!({
                    "cot": "req",
                    "data": {
                        "id": "222",
                        "manufacturer": "MAN02",
                        "vendor": "VEN02",
                        "order-code": "OC02",
                        "model": "MOD02",
                        "serial": "SER02",
                        "name": "NAM02",
                        "description": "DESC02",
                        "width": "W02",
                        "height": "H02",
                        "depth": "DEP02",
                        "weight": "WEI02"
                    },
                }),
                
            ),
            (
                03,
                "Device-333",
                json!({"dev-id": "Device-333"}),
                json!({
                    "cot": "req",
                    "data": {
                        "id": "333",
                        "manufacturer": "MAN03",
                        "vendor": "VEN03",
                        "order-code": "OC03",
                        "model": "MOD03",
                        "serial": "SER03",
                        "name": "NAM03",
                        "description": "DESC03",
                        "width": "W03",
                        "height": "H03",
                        "depth": "DEP03",
                        "weight": "WEI03"
                    },
                }),
                    
            ),
        ];
        let select_dev_info = SelectDevInfo::new(
            FakeDeviceInfo::new(
                test_data.clone().map(|(_, dev_id, _, val)| (dev_id, val)).into(),
            )
        );
        for (step, _, req, target) in test_data {
            let val = Query::new(step, Request::DeviceInfo, Cot::Req, req.as_object().unwrap().to_owned());
            //  {
            //     msg_id: step,
            //     map: req.as_object().unwrap().to_owned(),
            // };
            let result = select_dev_info.eval(val).unwrap().unwrap();
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        test_duration.exit();
    }
    ///
    /// Fake DeviceInfo for testing only
    pub struct FakeDeviceInfo {
        val: IndexMap<String, JsonVal>,
    }
    impl FakeDeviceInfo {
        fn new(val: Vec<(&str, JsonVal)>) -> Self {
            Self {
                val: IndexMap::from_iter(val.into_iter().map(|(dev_id, val)| {
                    (dev_id.to_owned(), val)
                })),
            }
        }
    }
    //
    //
    impl EvalEx<DevId, EvalResult> for FakeDeviceInfo {
        fn eval(&self, id: DevId) -> EvalResult {
        let error = Error::new("FakeDeviceInfo", "eval");
            match self.val.get(&(id.0)) {
                Some(val) => Ok(Some(json!(val))),
                None => Err(error.err(format!("id {} - is not found in the test_data", id.0))),
            }
        }
        fn exit(&self) {}
    }
}
