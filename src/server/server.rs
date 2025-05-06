use std::{net::TcpListener, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use coco::Stack;
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::thread_pool::{Scheduler, JoinHandle};
use crate::server::{ServerConf, Connection};

use super::{req_dev_info::ReqDevInfo, select_cot::SelectCot, select_req::SelectReq, Cot, Req};
///
/// The Server
/// - Setups socket server at specified address
/// - Spawnes `Connection` on each incoming requiest
pub struct Server {
    dbg: Dbg,
    conf: ServerConf,
    scheduler: Scheduler,
    connections: Arc<Stack<Connection>>,
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
            connections: Arc::new(Stack::new()),
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
        let connections = self.connections.clone();
        let exit = self.exit.clone();
        let handle = self.scheduler.spawn(move || {
            'main: loop {
                match TcpListener::bind(conf.address.clone()) {
                    Ok(listener) => {
                        for stream in listener.incoming() {
                            match stream {
                                Ok(stream) => {
                                    let conn = Connection::new(
                                        &dbg,
                                        conf.connection.clone(),
                                        stream,
                                        scheduler.clone(),
                                        SelectCot::new(
                                            vec![
                                                (Cot::Req, SelectReq::new(
                                                    vec![
                                                        (Req::DeviceInfo, ReqDevInfo::new()),
                                                    ]
                                                )),
                                            ],
                                        ),
                                    );
                                    match conn.run() {
                                        Ok(_) => connections.push(conn),
                                        Err(err) => log::warn!("{dbg}.run | Spawn connection error: {:?}", err),
                                    }
                                }
                                Err(err) => log::warn!("{dbg}.run | Get TcpStream error: {:?}", err),
                            }
                        }
                    }
                    Err(err) => log::warn!("{dbg}.run | Bind TcpServer error: {:?}", err),
                }
                std::thread::sleep(Duration::from_secs(1));
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
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
    ///
    /// Returns when internal thread's will finished
    pub fn wait(&self) -> Result<(), Error> {
        let error = Error::new(&self.dbg, "wait");
        while !self.connections.is_empty() {
            if let Some(conn) = self.connections.pop() {
                if let Err(err) = conn.wait() {
                    log::warn!("{}.wait | Bind TcpServer error: {:?}", self.dbg, err);
                }
            }
        }
        match self.handle.pop() {
            Some(handle) => handle.join().map_err(|err| error.pass(format!("{:?}", err))),
            None => Err(error.err("No handle")),
        }
    }
    ///
    /// Sends exit signal to main tread
    pub fn exit(&self) {
        let mut connections = vec![];
        while !self.connections.is_empty() {
            if let Some(conn) = self.connections.pop() {
                connections.push(conn);
            }
        }
        for conn in connections {
            conn.exit();
            self.connections.push(conn);
        }
        self.exit.store(true, Ordering::SeqCst);
    }
}