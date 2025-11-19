use std::{net::TcpListener, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::{collections::FxDashMap, sync::Handles, thread_pool::Scheduler};
use crate::server::{Connection, Cot, ServerConf};
use super::{select_cot::SelectCot, select_req::SelectReq, QueryId, SelectAct, SelectDevDoc, SelectDevInfo, DevStream};
///
/// The Server
/// - Setups socket server at specified address
/// - Spawnes `Connection` on each incoming requiest
pub struct Server {
    dbg: Dbg,
    conf: ServerConf,
    scheduler: Scheduler,
    connections: Arc<FxDashMap<String, Connection>>,
    listener: Arc<FxDashMap<usize, TcpListener>>,
    handles: Handles<()>,
    exit: Arc<AtomicBool>,
}
//
//
impl Server {
    ///
    /// Returns [Server] new instance
    pub fn new(parent: impl Into<String>, conf: ServerConf, scheduler: Scheduler) -> Self {
        let dbg = Dbg::new(parent.into(), "Server");
        Self {
            conf,
            scheduler,
            connections: Arc::new(FxDashMap::default()),
            listener: Arc::new(FxDashMap::default()),
            handles: Handles::new(&dbg),
            exit: Arc::new(AtomicBool::new(false)),
            dbg,
        }
    }
    ///
    /// [Server] Operation mode
    pub fn run(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let conf = self.conf.clone();
        let scheduler = self.scheduler.clone();
        let connections = self.connections.clone();
        let listeners = self.listener.clone();
        let exit = self.exit.clone();
        let handle = self.scheduler.spawn(move || {
            'main: loop {
                match TcpListener::bind(conf.address.clone()) {
                    Ok(listener) => {
                        listeners.insert(listeners.len(), listener.try_clone().unwrap());
                        for stream in listener.incoming() {
                            match stream {
                                Ok(stream) => {
                                    let client = stream.peer_addr().map(|a| a.to_string()).unwrap_or(connections.len().to_string());
                                    let conn = Connection::new(
                                        &dbg,
                                        conf.connection.clone(),
                                        stream,
                                        scheduler.clone(),
                                        //
                                        // Select handler for incomong messages by Cot
                                        SelectCot::new(
                                            vec![
                                                // Handling incomong messages with `Cot::Act` by field `cmd`
                                                (Cot::Act, Box::new(SelectAct::new(
                                                    vec![
                                                        // Handling incomong command `DeviceStream`
                                                        (QueryId::DeviceStream, Box::new(DevStream::new(
                                                            &dbg,
                                                            conf.dev_stream.clone(),
                                                            scheduler.clone(),
                                                        ))),
                                                    ]
                                                ))),
                                                // Handling incomong messages with Cot::Req by field `req`
                                                (Cot::Req, Box::new(SelectReq::new(
                                                    vec![
                                                        // Handling incomong request `DeviceInfo`
                                                        (QueryId::DeviceInfo, Box::new(SelectDevInfo::new("assets/info/"))),
                                                        // Handling incomong request `DeviceDoc`
                                                        (QueryId::DeviceDoc, Box::new(SelectDevDoc::new("assets/info/"))),
                                                    ]
                                                ))),
                                            ],
                                        ),
                                    );
                                    match conn.run() {
                                        Ok(_) => _ = connections.insert(client, conn),
                                        Err(err) => log::warn!("{dbg}.run | Spawn connection error: {:?}", err),
                                    }
                                    let keys: Vec<String> = connections.iter().map(|e| e.key().clone()).collect();
                                    for key in keys {
                                        if let Some(con) =  connections.get(&key) {
                                            if con.value().is_finished() {
                                                con.exit();
                                                connections.remove(&key);
                                            }
                                        }
                                    }
                                }
                                Err(err) => log::warn!("{dbg}.run | Get TcpStream error: {:?}", err),
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                        }
                    }
                    Err(err) => log::warn!("{dbg}.run | Bind TcpServer error: {:?}", err),
                }
                std::thread::sleep(Duration::from_secs(1));
                if exit.load(Ordering::SeqCst) {
                    for con in connections.iter() {
                        con.value().exit();
                    }
                    break 'main;
                }
            }
            Ok(())
        }).map_err(|err| Error::new(&self.dbg, "run").pass(err))?;
        self.handles.push(handle);
        Ok(())
    }
    ///
    /// Returns when internal thread's will finished
    #[allow(unused)]
    pub fn wait(&self) -> Result<(), Error> {
        for conn in self.connections.iter() {
            if let Err(err) = conn.wait() {
                log::warn!("{}.wait | Wait for TcpServer '{}' error: {:?}", self.dbg, conn.key(), err);
            }
        }
        self.handles.wait()
    }
    ///
    /// Sends exit signal to main tread
    #[allow(unused)]
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        for listener in self.listener.iter() {
            if let Err(err) = listener.set_nonblocking(true) {
                log::warn!("{}.wait | TcpListener '{}' set_nonblocking error: {:?}", self.dbg, listener.key(), err);
            }
        }
        for conn in self.connections.iter() {
            conn.exit();
        }
    }
}
