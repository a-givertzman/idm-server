use std::{io::{BufReader, BufWriter, Read, Write}, net::{Shutdown, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, Arc}};
use api_tools::api::message::{
    fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn},
    message::{MessageField, MessageParse}, message_kind::MessageKind, parse_data::ParseData,
    parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn,
};
use coco::Stack;
use sal_core::{dbg::Dbg, error::Error};
use sal_sync::thread_pool::{Scheduler, JoinHandle};
use crate::{
    device_info::DevId, domain::{Eval, TcpMessage}, server::ConnectionConf
};

use super::{BytesCtx, JsonCtx, Reply};

///
/// The [Connection] of the `Server`
pub struct Connection {
    dbg: Dbg,
    conf: ConnectionConf,
    stream: Stack<TcpStream>,
    scheduler: Scheduler,
    ctx: Stack<Box<dyn Eval<BytesCtx, Result<JsonCtx, Error>> + Send>>,
    handle: Stack<JoinHandle<()>>,
    exit: Arc<AtomicBool>,
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
        ctx: impl Eval<BytesCtx, Result<JsonCtx, Error>> + Send + 'static,
    ) -> Self {
        let stream_ = Stack::new();
        stream_.push(stream);
        let ctx_: Stack<Box<dyn Eval<BytesCtx, Result<JsonCtx, Error>> + Send + 'static>> = Stack::new();
        ctx_.push(Box::new(ctx));
        Self {
            dbg: Dbg::new(parent.into(), "Connection"),
            conf,
            stream: stream_,
            scheduler,
            ctx: ctx_,
            handle: Stack::new(),
            exit: Arc::new(AtomicBool::new(false)),
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
    /// [Connection] Operation mode
    pub fn run(&self) -> Result<(), Error> {
        let dbg = self.dbg.clone();
        let conf = self.conf.clone();
        let stream = self.stream.pop().unwrap();
        let mut ctx = self.ctx.pop().unwrap();
        let mut w_stream = BufWriter::new(stream.try_clone().unwrap());
        let mut r_stream = BufReader::new(stream.try_clone().unwrap());
        let exit = self.exit.clone();
        let handle = self.scheduler.spawn(move || {
            let error = Error::new("Connection", "run");
            let mut message = Self::tcp_message(&dbg);
            let mut buf = [0u8; 1024 * 4];
            'main: loop {
                match r_stream.read(&mut buf) {
                    Ok(len) => {
                        match message.parse(buf[..len].to_owned()) {
                            Ok((id, kind, _, bytes)) => {
                                // TODO: parsed bytes should be converted into `Requiest` ( bytes -> String -> serde_json::Value )
                                // parsed incoming json request:
                                // {
                                //      cot: Cot::Act,
                                //      data: null,
                                //      err: null
                                // }
                                //  json reply:
                                // {
                                //      cot: Cot: Inf,
                                //      data: {
                                //          Device info data
                                //      },
                                //      err: {
                                //          message: "Error message"
                                //      }
                                // }
                                match kind {
                                    MessageKind::Bytes => {
                                        let reply = match ctx.eval(BytesCtx { bytes, id: DevId(id.0) }) {
                                            Ok(reply) => Reply {
                                                id: reply.id.0,
                                                data: reply.value,
                                                error: None,
                                            },
                                            Err(err) => Reply {
                                                id: id.0,
                                                data: serde_json::Value::Null,
                                                error: Some(super::ReplyError {
                                                    message: error.pass(err).to_string()
                                                }),
                                            }
                                        };
                                        match serde_json::to_vec(&reply) {
                                            Ok(bytes) => {
                                                let bytes = message.build(&bytes, id.0);
                                                if let Err(err) = w_stream.write_all(&bytes) {
                                                    log::warn!("{dbg}.run | TcpStream write error: {:?}", err);
                                                    if let Err(err) = Self::close(&dbg, stream) {
                                                        log::warn!("{dbg}.run | Close tcp stream error: {:?}", err);
                                                    }
                                                    break 'main;
                                                }
                                            }
                                            Err(err) => {
                                                log::warn!("{dbg}.run | Serialize reply error: {:?}", err);
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
                            if let Err(err) = Self::close(&dbg, stream) {
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
    pub fn close(dbg: &Dbg, stream: TcpStream) -> Result<(), Error> {
        stream
            .shutdown(Shutdown::Both)
            .map_err(|err| Error::new(dbg, "close").pass(err.to_string()))
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

///
/// Connection status
enum IsConnected<T, E> {
    Active(T),
    Closed(E),
}
