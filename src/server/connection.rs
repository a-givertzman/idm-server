use std::{io::{BufReader, BufWriter, Read, Write}, net::{Shutdown, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use api_tools::api::message::{
    fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn},
    message::{MessageField, MessageParse}, message_kind::MessageKind, parse_data::ParseData,
    parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn,
};
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::{sync::{Handles, Owner}, thread_pool::{JoinHandle, Scheduler}};
use crate::{
    domain::{Eval, Hub, JsonVal, Link, TcpMessage}, server::ConnectionConf
};
use super::{BytesCtx, Event, JsonCtx, Reply};

///
/// The [Connection] of the `Server`
pub struct Connection {
    dbg: Dbg,
    conf: ConnectionConf,
    stream: Owner<TcpStream>,
    scheduler: Scheduler,
    ctx: Owner<Box<dyn Eval<(BytesCtx, Option<Link>), Result<JsonCtx, Error>> + Send>>,
    handles: Handles<()>,
    exit: Arc<AtomicBool>,
    hub_exit: Arc<AtomicBool>,
}
//
// 
impl Connection {
    ///
    /// Returns [Connection] new instance
    pub fn new(
        parent: impl Into<String>,
        conf: ConnectionConf,
        stream: TcpStream,
        scheduler: Scheduler,
        ctx: impl Eval<(BytesCtx, Option<Link>), Result<JsonCtx, Error>> + Send + 'static,
    ) -> Self {
        let dbg = Dbg::new(parent.into(), "Connection");
        Self {
            conf,
            stream: Owner::new(stream),
            scheduler,
            ctx: Owner::new(Box::new(ctx)),
            handles: Handles::new(&dbg),
            exit: Arc::new(AtomicBool::new(false)),
            hub_exit: Arc::new(AtomicBool::new(false)),
            dbg,
        }
    }
    ///
    /// Setups TCP Message
    fn tcp_message(dbg: &Dbg) -> TcpMessage {
        TcpMessage::new(
            dbg,
            vec![
                MessageField::Syn(FieldSyn::default()),
                MessageField::Id(FieldId(4)),
                MessageField::Kind(FieldKind(MessageKind::Bytes)),
                MessageField::Size(FieldSize(4)),
                MessageField::Data(FieldData(vec![]))
            ],
            ParseData::new(
                dbg,
                ParseSize::new(
                    dbg,
                    FieldSize(4),
                    ParseKind::new(
                        dbg,
                        FieldKind(MessageKind::Bytes),
                        ParseId::new(
                            dbg,
                            FieldId(4),
                            ParseSyn::new(
                                dbg,
                                FieldSyn::default(),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }
    ///
    /// - Sets the read timeout to the timeout specified.
    /// - Sets the write timeout to the timeout specified.
    fn set_tcp_timeout(&self, stream: &TcpStream, timeout: Duration) {
        if let Err(err) = stream.set_read_timeout(Some(timeout)) {
            log::warn!("{}.set_tcp_timeout | Set read timeout error: {:?}", self.dbg, err);
        }
        if let Err(err) = stream.set_write_timeout(Some(timeout)) {
            log::warn!("{}.set_tcp_timeout | Set write timeout error: {:?}", self.dbg, err);
        }
    }
    ///
    /// [Connection] Operation mode
    pub fn run(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let error = Error::new(&self.dbg, "run");
        let conf = self.conf.clone();
        let stream = self.stream.take().ok_or(error.err(format!("Can't take ctx")))?;
        self.set_tcp_timeout(&stream, conf.timeout);
        let mut ctx = self.ctx.take().ok_or(error.err(format!("Can't take ctx")))?;
        let mut r_stream = BufReader::new(stream.try_clone().unwrap());
        let hub = Arc::new(Hub::new(&dbg, Some(self.hub_exit.clone())));
        self.send(&stream, hub.clone())?;
        let exit = self.exit.clone();
        let handle = self.scheduler.spawn(move || {
            let error = Error::new(&dbg, "run");
            let link = hub.link();
            let mut message = Self::tcp_message(&dbg);
            let mut buf = [0u8; 1024 * 4];
            'main: loop {
                match r_stream.read(&mut buf) {
                    Ok(len) => {
                        match message.parse(buf[..len].to_owned()) {
                            Ok((msg_id, kind, _, bytes)) => {
                                match kind {
                                    MessageKind::Bytes => {
                                        let reply = match ctx.eval((BytesCtx { bytes, msg_id: msg_id.0 }, Some(hub.link()))) {
                                            Ok(reply) => {
                                                match reply.is_empty {
                                                    true => None,
                                                    false => Some(Reply {
                                                        data: reply.value,
                                                        error: None,
                                                    }),
                                                }
                                            }
                                            Err(err) => Some(Reply {
                                                data: JsonVal::Null,
                                                error: Some(super::ReplyError {
                                                    message: error.pass(err).to_string()
                                                }),
                                            })
                                        };
                                        if let Some(reply) = reply {
                                            match serde_json::to_vec(&reply) {
                                                Ok(bytes) => {
                                                    if let Err(err) = link.send(Event { msg_id: msg_id.0, bytes }) {
                                                        log::warn!("{dbg}.run | Send reply error: {:?}", err);
                                                    }
                                                }
                                                Err(err) => {
                                                    log::warn!("{dbg}.run | Serialize reply error: {:?}", err);
                                                }
                                            }
                                        }
                                    }
                                    _ => log::warn!("{dbg}.run | Message of kind '{:?}' - is not supported", kind),
                                }
                            }
                            Err(err) => {
                                log::warn!("{dbg}.run | parse error: {:?}", err)
                            }
                        }
                    }
                    Err(err) => {
                        log::warn!("{}.run | TcpStream read error: {:?}", dbg, err);
                        if let IsConnected::Closed(_) = Self::parse_err(&dbg, err) {
                            if let Err(err) = Self::close(&dbg, &stream) {
                                log::warn!("{dbg}.run | Close tcp stream error: {:?}", err);
                            }
                        }
                        break 'main;
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
            Ok(())
        })?;
        self.handles.push(handle);
        Ok(())
    }
    ///
    /// Sends messages to the TCP Socket
    fn send(&self, stream: &TcpStream, hub: Arc<Hub>) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let error = Error::new(&dbg, "send");
        let stream = stream.try_clone().map_err(|err| error.pass_with(format!("stream.try_clone error"), err.to_string()))?;
        let hub_exit = self.hub_exit.clone();
        let handle = hub.listen::<Event, Option<()>>(self.scheduler.clone(), move |event: Event, _| {
            let mut w_stream = BufWriter::new(&stream);
            let mut message = Self::tcp_message(&dbg);
            let bytes = message.build(&event.bytes, event.msg_id);
            if let Err(err) = w_stream.write_all(&bytes) {
                log::warn!("{dbg}.run | TcpStream write error: {:?}", err);
                if let Err(err) = Self::close(&dbg, &stream) {
                    log::warn!("{dbg}.run | Close tcp stream error: {:?}", err);
                }
                hub_exit.store(true, Ordering::SeqCst);
            }
            None
        })?;
        self.handles.push(handle);
        Ok(())
    }
    ///
    /// Returns Connection status dipending on IO Error
    fn parse_err(dbg: &Dbg, input: std::io::Error) -> IsConnected<(), Error> {
        // log::warn!("{}.parse_err | error reading from socket: {:?}", dbg, input);
        // log::warn!("{}.parse_err | error kind: {:?}", dbg, input.kind());
        let err = Error::new(dbg, "parse_err").pass(&input.to_string());
        match input.kind() {
            // std::io::ErrorKind::NotFound => todo!(),
            std::io::ErrorKind::PermissionDenied => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionRefused => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionReset => IsConnected::Closed(err),
            std::io::ErrorKind::HostUnreachable => IsConnected::Closed(err),
            std::io::ErrorKind::NetworkUnreachable => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionAborted => IsConnected::Closed(err),
            std::io::ErrorKind::NotConnected => IsConnected::Closed(err),
            std::io::ErrorKind::AddrInUse => IsConnected::Closed(err),
            std::io::ErrorKind::AddrNotAvailable => IsConnected::Closed(err),
            std::io::ErrorKind::NetworkDown => IsConnected::Closed(err),
            std::io::ErrorKind::BrokenPipe => IsConnected::Closed(err),
            std::io::ErrorKind::AlreadyExists => IsConnected::Closed(err),
            std::io::ErrorKind::WouldBlock => IsConnected::Closed(err),
            // std::io::ErrorKind::NotADirectory => todo!(),
            // std::io::ErrorKind::IsADirectory => todo!(),
            // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
            // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
            // std::io::ErrorKind::FilesystemLoop => todo!(),
            // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
            // std::io::ErrorKind::InvalidInput => todo!(),
            // std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => IsConnected::Closed(err),
            // std::io::ErrorKind::WriteZero => todo!(),
            // std::io::ErrorKind::StorageFull => todo!(),
            // std::io::ErrorKind::NotSeekable => todo!(),
            // std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
            // std::io::ErrorKind::FileTooLarge => todo!(),
            // std::io::ErrorKind::ResourceBusy => todo!(),
            // std::io::ErrorKind::ExecutableFileBusy => todo!(),
            // std::io::ErrorKind::Deadlock => todo!(),
            // std::io::ErrorKind::CrossesDevices => todo!(),
            // std::io::ErrorKind::TooManyLinks => todo!(),
            // std::io::ErrorKind::InvalidFilename => todo!(),
            // std::io::ErrorKind::ArgumentListTooLong => todo!(),
            // std::io::ErrorKind::Interrupted => todo!(),
            // std::io::ErrorKind::Unsupported => todo!(),
            // std::io::ErrorKind::UnexpectedEof => todo!(),
            // std::io::ErrorKind::OutOfMemory => todo!(),
            // std::io::ErrorKind::Other => todo!(),
            _ => IsConnected::Closed(err),
        }
    }
    ///
    /// Closes a connection
    pub fn close(dbg: &Dbg, stream: &TcpStream) -> Result<(), Error> {
        stream
            .shutdown(Shutdown::Both)
            .map_err(|err| Error::new(dbg, "close").pass(err.to_string()))
    }
    ///
    /// Returns when internal thread's will finished
    #[allow(unused)]
    pub fn wait(&self) -> Result<(), Error> {
        self.handles.wait()
    }
    ///
    /// Sends exit signal to main tread
    pub fn exit(&self) {
        self.hub_exit.store(true, Ordering::SeqCst);
        self.exit.store(true, Ordering::SeqCst);

    }
}
///
/// Connection status
enum IsConnected<T, E> {
    #[allow(unused)]
    Active(T),
    Closed(E),
}
