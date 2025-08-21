use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use rand::Rng;
use sal_core::dbg::Dbg;
use sal_sync::{services::ServiceCycle, sync::Handles, thread_pool::Scheduler};
use serde::{Deserialize, Serialize};
use crate::{domain::{Error, EvalEx, Link}, server::{EvalResult, Event, Query, Request}};
use super::{DevConf, DevStreamConf};

///
/// Producess Device's events
pub struct DevStream {
    dbg: Dbg,
    conf: DevStreamConf,
    scheduler: Scheduler,
    is_active: Arc<AtomicBool>,
    handle: Handles<()>,
    exit: Arc<AtomicBool>,
}
//
//
impl DevStream {
    ///
    /// Returns [SelectDevStream] new instance
    pub fn new(
        parent: impl Into<String>,
        conf: DevStreamConf,
        scheduler: Scheduler,
    ) -> Self {
        let dbg = Dbg::new(parent.into(), "DevStream");
        Self {
            conf,
            scheduler,
            is_active: Arc::new(AtomicBool::new(false)),
            handle: Handles::new(&dbg),
            exit: Arc::new(AtomicBool::new(false)),
            dbg,
        }
    }
    ///
    /// Wait for inner thread being finished
    pub fn wait(&self) -> Result<(), Error> {
        self.handle.wait()
    }
}
//
//
impl EvalEx<(Query<Request>, Option<Link>), EvalResult> for DevStream {
    fn eval(&self, (_query, link): (Query<Request>, Option<Link>)) -> EvalResult {
        let error = Error::new("DevStream", "eval");
        match self.is_active.load(Ordering::SeqCst) {
            true => Ok(None),
            false => match link {
                Some(link) => {
                    let dbg = self.dbg.clone();
                    let conf = self.conf.clone();
                    let exit = self.exit.clone();
                    log::info!("{dbg}.eval | Staring...");
                    let handle = self.scheduler.spawn(move || {
                        let mut devices: Vec<Device> = conf.devices.iter().map(|(id, conf)| {
                            Device::new(
                                id.to_owned(),
                                format!("On"),
                                conf.value,
                                conf.to_owned(),
                            )
                        }).collect();
                        log::debug!("{dbg}.eval | Configured {} devices", devices.len());
                        let mut cycle = ServiceCycle::new(&dbg.to_string(), Duration::from_millis(10));
                        'main: loop {
                            cycle.start();
                            for dev in &mut devices {
                                if dev.elapsed() > dev.conf.interval {
                                    *dev = Device::from(dev.clone());
                                    match serde_json::to_vec(&dev) {
                                        Ok(bytes) => {
                                            log::trace!("{dbg}.eval | Sending dev.id: {}", dev.id);
                                            // log::warn!("{dbg}.eval | Sending dev: {:?}", String::from_utf8_lossy(&bytes));
                                            let event = Event::new(dev.id.parse().unwrap(), bytes);
                                            // log::warn!("{dbg}.eval | Sending event: {:?}", event);
                                            if let Err(err) = link.send(event) {
                                                log::warn!("{dbg}.eval | Send error: {:?}", err);
                                            }
                                        },
                                        Err(err) => log::warn!("{dbg}.eval | Json error: {:?}", err),
                                    }
                                }
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                            cycle.wait();
                        }
                        log::info!("{dbg}.eval | Exit");
                        Ok(())
                    })
                        .map_err(|err| Error::new(&self.dbg, "eval").pass(err))?;
                    let dbg = self.dbg.clone();
                    self.handle.push(handle);
                    log::info!("{dbg}.eval | Staring - Ok");
                    Ok(None)
                }
                None => Err(error.err("Link is missing")),
            },
        }
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
unsafe impl Send for DevStream {}
///
/// Device stream info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    /// Like On/Off
    pub state: String,
    /// Like speed, current, etc...
    pub value: String,
    pub conf: DevConf,
    #[serde(skip)]
    pub val: f64,
    #[serde(skip)]
    interval: Option<Instant>,
}
impl Device {
    ///
    /// Returns [Device] new instance
    pub fn new(id: String, state: String, val: f64, conf: DevConf) -> Self {
        Self {
            id,
            state,
            value: format!("{:.3}", val),
            conf,
            val,
            interval: Some(Instant::now()),
        }
    }
    ///
    /// Elapsed from las send of [Device] 
    pub fn elapsed(&self) -> Duration {
        match self.interval {
            Some(interval) => interval.elapsed(),
            None => panic!("Device({}).elapsed | Interval is not initialized", self.id),
        }
    }
}
//
//
impl From<&Device> for Device {
    ///
    /// Returns [Device] created from it prevouse state
    /// using configured `conf.diviation`
    fn from(dev: &Device) -> Self {
        let mut rng = rand::rng();
        let sign = rng.random::<bool>();
        let deviation = dev.conf.deviation * 0.1 * rng.random::<f64>();
        let val = match sign {
            true => dev.val + deviation,
            false => dev.val - deviation,
        };
        Self {
            id: dev.id.to_owned(),
            state: dev.state.to_owned(),
            value: format!("{:.3}", val),
            conf: dev.conf.to_owned(),
            val,
            interval: Some(Instant::now()),
        }
    }
}