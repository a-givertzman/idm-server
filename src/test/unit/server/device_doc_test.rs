#[cfg(test)]

use std::{sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use serde::{Deserialize, Serialize};
use serde_json::json;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::{device::{DevId, DeviceDoc}, domain::EvalEx, server::DeviceInfoRequest};
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
    let path = "src/test/unit/server/device_doc/";
    let test_data = [
        (
            01,
            format!("111"),
            r#"# Model KEAZ.336023

Реле перегрузки OptiStart TF-MAP-1,5 для мини-контактора серии OptiStart K-M\nРеле чувствительно к обрыву фазы\nМаксимальный ток 6 А (230В)

- id: 111
- manufacturer: КЭАЗ
- vendor: КЭАЗ
- order-code: 336023
- serial: OptiStart TF-M
- name: Реле перегрузки
- width: 44
- height: 36
- depth: 89,2
- weight: 0,06"#,
        ),
        (
            02,
            format!("222"),
            r#"# Model: EKF.ed16-22bms-24

24В (AC/DC), цвет красный, прерывистный сигнал

- id: 222
- manufacturer: EKF
- vendor: EKF
- order-code: ed16-22bms-24
- serial: ED16
- name: Зуммер
- width: 28
- height: 28
- depth: 50
- weight: 0,1"#,
        ),
    ];
    let dev_doc = DeviceDoc::from_path(path);
    for (step, id, target) in test_data {
        let target = json!({"doc": target});
        let result = dev_doc.eval(DevId(id)).unwrap().unwrap();
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
