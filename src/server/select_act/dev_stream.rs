use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use bincode::{Decode, Encode};
use coco::Stack;
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::{JoinHandle, Scheduler};
use serde::{Deserialize, Serialize};
use crate::{domain::{Error, Eval, Link}, server::{Event, MapCtx}};
use super::DevStreamConf;

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
    fn eval(&mut self, (input, link): (MapCtx, Option<Link>)) -> Result<(), Error> {
        let error = Error::new("DevStream", "eval");
        match link {
            Some(link) => {
                let dbg = self.dbg.clone();
                let conf = self.conf.clone();
                let exit = self.exit.clone();
                log::warn!("{dbg}.run | Staring...");
                let handle = self.scheduler.spawn(move || {
                    'main: loop {
                        for (id, dev_conf) in &conf.devices {
                            let dev_state = String::new();
                            let dev_value = String::new();
                            let bytes = serde_json::to_vec(&Device { id: id.to_owned(), state: dev_state, value: dev_value });
                            if let Err(err) = link.send(Ok::<_, Error>(Event { msg_id: input.id.0, bytes: vec![] })) {
                                log::warn!("{dbg}.run | Close tcp stream error: {:?}", err);
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
    id: String,
    /// Like On/Off
    state: String,
    /// Like speed, current, etc...
    value: String,
}