use std::{fmt::Debug, sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
use sal_core::dbg::Dbg;
use sal_sync::{services::ServiceCycle, sync::Handles, thread_pool::Scheduler};
use crate::{device::Device, domain::{Error, EvalEx, Link}, server::{EvalResult, Frame, Query, Reply}};
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
impl<K: Debug + Copy + bincode::Encode + Send + 'static> EvalEx<(Frame<K>, Option<Link>), EvalResult<K>> for DevStream {
    fn eval(&self, (frame, link): (Frame<K>, Option<Link>)) -> EvalResult<K> {
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
                        //
                        // Prepare event & send Event
                        match frame.operation::<Query>() {
                            Ok(query) => {
                                log::debug!("{dbg}.eval | Parsed: {:?}", query);
                            }
                            Err(err) => {
                                let frame = Frame::from(&dbg, frame.reply_err(err.to_string()));
                                if let Err(err) = link.send(frame) {
                                    log::warn!("{dbg}.eval | Send error: {:?}", err);
                                    return Ok(());
                                }
                            }
                        }

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
                        'main: while !exit.load(Ordering::Acquire) {
                            cycle.start();
                            for dev in &mut devices {
                                if dev.elapsed() > dev.conf.interval {
                                    *dev = Device::from(dev.clone());
                                    let frame = Frame::from(&dbg,
                                        frame.reply(Reply::DeviceStream(dev.clone())),
                                    );
                                    if let Err(err) = link.send(frame) {
                                        log::warn!("{dbg}.eval | Send error: {:?}", err);
                                    }
                                }
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
    ///
    /// Halts hanbler
    fn exit(&self) {
        self.exit.store(true, Ordering::Release);
    }
}
//
//
unsafe impl Send for DevStream {}
