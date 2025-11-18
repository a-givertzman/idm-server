#[cfg(test)]

use std::{collections::HashMap, sync::Once, time::Duration};
use sal_core::dbg::Dbg;
use sal_sync::{services::entity::Cot, thread_pool::ThreadPool};
use serde_json::json;
use testing::stuff::max_test_duration::TestDuration;
use debugging::session::debug_session::{DebugSession, LogLevel};
use crate::{domain::{EvalEx, Link}, server::{DevStream, DevStreamConf, Device, Event, Query, Request}};
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
    let dbg = Dbg::own("DevStream-test");
    log::debug!("\n{}", dbg);
    let test_duration = TestDuration::new(&dbg, Duration::from_secs(5));
    test_duration.run().unwrap();
    // let test_data = [
    // ];
    let thread_pool = ThreadPool::new(&dbg, Some(3));
    let conf = r#"
        devices:
            01:
                value: 10.0
                deviation: 3.0
                interval: 200   # ms
            02:
                value: 20.0
                deviation: 3.0
                interval: 300   # ms
            03:
                value: 30.0
                deviation: 3.0
                interval: 400   # ms
    "#;
    let conf: DevStreamConf = serde_yaml::from_str(conf).unwrap();
    let dev_stream = DevStream::new(&dbg, conf.clone(), thread_pool.scheduler());
    let (loc, rem) = Link::split(&dbg);
    let val = Query::new(0, Request::DeviceStream, Cot::Act, json!({}).as_object().unwrap().to_owned());
    dev_stream.eval((val, Some(rem))).unwrap();
    let mut results = HashMap::new();
    loop {
        match loc.recv_timeout::<Event>(Duration::from_millis(300)) {
            Ok(event) => match event {
                Some(event) => {
                    log::trace!("{dbg} | event.id {}", event.msg_id);
                    log::trace!("{dbg} | event.bytes {:?}", String::from_utf8_lossy(&event.bytes));
                    match serde_json::from_slice::<Device>(&event.bytes) {
                        Ok(dev) => {
                            log::trace!("dev: {}", dev.id);
                            results.insert(dev.id.clone(), dev);
                        }
                        Err(err) => log::warn!("Parse dev error {:?} from: \n\t{:#?}", err, String::from_utf8_lossy(&event.bytes)),
                    }
                }
                None => {
                    panic!("Empty event - Receive timeout");
                }
            }
            Err(err) => {
                log::warn!("recv error {:?}", err);
                break;
            }
        }
        if results.len() >= 3 { break };
    }
    dev_stream.exit();
    dev_stream.wait().unwrap();
    test_duration.exit();
}
