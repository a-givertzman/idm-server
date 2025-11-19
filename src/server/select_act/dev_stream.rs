use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use sal_core::dbg::Dbg;
use sal_sync::{services::ServiceCycle, sync::Handles, thread_pool::Scheduler};
use crate::{device::Device, domain::{Error, EvalEx, Link}, server::{EvalResult, Event, Query, QueryId, Request}};
use super::DevStreamConf;

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
    #[allow(unused)]
    pub fn wait(&self) -> Result<(), Error> {
        self.handle.wait()
    }
}
//
//
impl EvalEx<(Request<QueryId, Query>, Option<Link>), EvalResult> for DevStream {
    fn eval(&self, (req, link): (Request<QueryId, Query>, Option<Link>)) -> EvalResult {
        let error = Error::new("DevStream", "eval");
        let event_id = req.event_id;
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
                                            let event = Event::new(event_id, bytes);
                                            // log::warn!("{dbg}.eval | Sending event: {:?}", event);
                                            if let Err(err) = link.send(event) {
                                                log::warn!("{dbg}.eval | Send error: {:?}", err);
                                            }
                                        },
                                        Err(err) => log::warn!("{dbg}.eval | Json error: {:?}", err),
                                    }
                                }
                            }
                            if exit.load(Ordering::Acquire) {
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
        self.exit.store(true, Ordering::Release);
    }
}
//
//
unsafe impl Send for DevStream {}
