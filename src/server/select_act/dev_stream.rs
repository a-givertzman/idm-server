use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, Instant}};
use coco::Stack;
use rand::Rng;
use sal_core::dbg::Dbg;
use sal_sync::{services::ServiceCycle, thread_pool::{JoinHandle, Scheduler}};
use serde::{Deserialize, Serialize};
use crate::{domain::{Error, Eval, Link}, server::{Event, MapCtx}};
use super::{DevConf, DevStreamConf};

///
/// Producess Device's events
pub struct DevStream {
    dbg: Dbg,
    conf: DevStreamConf,
    scheduler: Scheduler,
    is_active: Arc<AtomicBool>,
    handle: Stack<JoinHandle<()>>,
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
        Self {
            dbg: Dbg::new(parent.into(), "DevStream"),
            conf,
            scheduler,
            is_active: Arc::new(AtomicBool::new(false)),
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Wait for inner thread being finished
    pub fn wait(&self) -> Result<(), Error> {
        log::warn!("{}.цфше | Checking threads...", self.dbg);
        while !self.handle.is_empty() {
            log::warn!("{}.wait | Some threads found, waiting...", self.dbg);
            match self.handle.pop() {
                Some(h) => {
                    let error = Error::new("DevStream", "wait");
                    h.join().map_err(|err| error.pass(err)).unwrap();
                }
                _ => break,
            }
        }
        Ok(())
    }
    ///
    /// Send exit signal to the spawned thread
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for DevStream {
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("DevStream", "eval");
        match self.is_active.load(Ordering::SeqCst) {
            true => Ok(()),
            false => match link {
                Some(link) => {
                    let dbg = self.dbg.clone();
                    let conf = self.conf.clone();
                    let exit = self.exit.clone();
                    log::warn!("{dbg}.eval | Staring...");
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
                                            log::warn!("{dbg}.eval | Sending dev.id: {}", dev.id);
                                            // log::warn!("{dbg}.eval | Sending dev: {:?}", String::from_utf8_lossy(&bytes));
                                            let event = Event { msg_id: input.msg_id, bytes };
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
                        log::warn!("{dbg}.eval | Exit");
                        Ok(())
                    });
                    let dbg = self.dbg.clone();
                    let error = Error::new(&self.dbg, "eval");
                    match handle {
                        Ok(handle) => {
                            self.handle.push(handle);
                            log::warn!("{dbg}.eval | Staring - Ok");
                            Ok(())
                        }
                        Err(err) => Err(error.pass(err)),
                    }
                }
                None => Err(error.err("Link is missing")),
            },
        }
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