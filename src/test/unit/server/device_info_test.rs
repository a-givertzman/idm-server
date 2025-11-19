#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use serde::{Deserialize, Serialize};
use serde_json::json;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::{device::{DevId, DeviceInfo}, domain::EvalEx, server::DeviceInfoQuery};
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
    let dbg = Dbg::own("device_info.eval");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
    test_duration.run().unwrap();
    let path = "src/test/unit/server/device_info/";
    let test_data = [
        (
            01,
            format!("111"),
            json!({
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
            }),
        ),
        (
            02,
            format!("222"),
            json!({
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
            }),
        ),
        (
            03,
            format!("333"),
            json!({
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
            }),
        ),
    ];
    let dev_info = DeviceInfo::from_path(path);
    for (step, id, target) in test_data {
        let target: DeviceInfo = serde_json::from_value(target).unwrap();
        let result = dev_info.eval(DevId(id)).unwrap().unwrap();
        let result: DeviceInfo = serde_json::from_value(result).unwrap();
        assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    }
    // assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
    test_duration.exit();
}
///
/// Fake Request
#[derive(Debug, Serialize, Deserialize)]
struct FakeRequest {
    data: DeviceInfoQuery
}
