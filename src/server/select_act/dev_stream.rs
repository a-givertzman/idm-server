use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use coco::Stack;
use rand::Rng;
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::{JoinHandle, Scheduler};
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
}
//
//
impl Eval<(MapCtx, Option<Link>), Result<(), Error>> for DevStream {
    fn eval(&mut self, (_, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("DevStream", "eval");
        match self.is_active.load(Ordering::SeqCst) {
            true => Ok(()),
            false => match link {
                Some(link) => {
                    let dbg = self.dbg.clone();
                    let conf = self.conf.clone();
                    let exit = self.exit.clone();
                    log::warn!("{dbg}.run | Staring...");
                    let handle = self.scheduler.spawn(move || {
                        let devices: Vec<Device> = conf.devices.iter().map(|(id, conf)| {
                            Device::new(
                                id.to_owned(),
                                format!("On"),
                                conf.value,
                                conf.to_owned(),
                            )
                        }).collect();
                        'main: loop {
                            for dev in &devices {
                                let dev = Device::from(dev);
                                match serde_json::to_vec(&dev) {
                                    Ok(bytes) => {
                                        if let Err(err) = link.send(Ok::<_, Error>(Event { msg_id: input.id.0, bytes })) {
                                            log::warn!("{dbg}.run | Send error: {:?}", err);
                                        }
                                    },
                                    Err(err) => log::warn!("{dbg}.run | Json error: {:?}", err),
                                }
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                        }
                        log::warn!("{dbg}.run | Exit");
                        Ok(())
                    });
                    let dbg = self.dbg.clone();
                    let error = Error::new(&self.dbg, "run");
                    match handle {
                        Ok(handle) => {
                            self.handle.push(handle);
                            log::warn!("{dbg}.run | Staring - Ok");
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    /// Like On/Off
    pub state: String,
    /// Like speed, current, etc...
    pub value: String,
    #[serde(skip)]
    pub val: f64,
    pub conf: DevConf,
}
impl Device {
    ///
    /// Returns [Device] new instance
    fn new(id: String, state: String, val: f64, conf: DevConf) -> Self {
        Self {
            id,
            state,
            value: format!("{:.3}", val),
            val,
            conf,
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
            val,
            conf: dev.conf.to_owned(),
        }
    }
}