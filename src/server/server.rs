use std::{net::TcpListener, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use coco::Stack;
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::thread_pool::{scheduler::Scheduler, JoinHandle};

use super::server_conf::ServerConf;

pub struct Server {
    dbg: Dbg,
    conf: ServerConf,
    scheduler: Scheduler,
    handle: Stack<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
}
//
//
impl Server {
    ///
    /// Returns [Server] new instance
    pub fn new(parent: impl Into<String>, conf: ServerConf, scheduler: Scheduler) -> Self {
        Self {
            dbg: Dbg::new(parent.into(), "Server"),
            conf,
            scheduler,
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// [Server] Operation mode
    pub fn run(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let conf = self.conf.clone();
        let scheduler = self.scheduler.clone();
        let exit = self.exit.clone();
        let handle = self.scheduler.spawn(move || {
            'main: loop {
                match TcpListener::bind(conf.address.clone()) {
                    Ok(listener) => {
                        for stream in listener.incoming() {
                            match stream {
                                Ok(stream) => {
                                    scheduler.spawn(move|| {
                                        
                                        Ok(())
                                    });
                                }
                                Err(_) => todo!(),
                            }
                        }
                    }
                    Err(_) => todo!(),
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
            Ok(())
        });
        match handle {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
    ///
    /// Returns when internal thread's will finished
    pub fn wait(&self) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "wait");
        match self.handle.pop() {
            Some(handle) => handle.join().map_err(|err| error.pass(format!("{:?}", err))),
            None => Err(error.err("No handle")),
        }
    }
    ///
    /// Sends exit signal to main tread
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}