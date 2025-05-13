use std::sync::{atomic::AtomicBool, Arc};
use coco::Stack;
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::{JoinHandle, Scheduler};
use crate::{domain::{Error, Eval, Link}, server::MapCtx};
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
            dbg: Dbg::new(parent.into(), "Connection"),
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
        let error = Error::new("SelectDevDoc", "eval");
        let handle = self.scheduler.spawn(move || {
            Ok(())
        });
        let error = Error::new(&self.dbg, "run");
        match handle {
            Ok(handle) => {
                self.handle.push(handle);
                Ok(())
            }
            Err(err) => Err(error.pass(err)),
        }
    }
}
//
//
unsafe impl Send for DevStream {}
