use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use coco::Stack;
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::{JoinHandle, Scheduler};
use crate::{domain::{Error, Eval, Link}, server::{Event, MapCtx, Reply}};
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
        let dbg = self.dbg.clone();
        match link {
            Some(link) => {
                let exit = self.exit.clone();
                log::warn!("{dbg}.run | Staring...");
                let handle = self.scheduler.spawn(move || {
                    'main: loop {
                        let event = Event {
                            id: input.id.0,
                            bytes: vec![],
                        };
                        if let Err(err) = link.send(Ok::<_, Error>(event)) {
                            log::warn!("{dbg}.run | Close tcp stream error: {:?}", err);
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
