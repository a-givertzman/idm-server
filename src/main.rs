mod conf;
mod server;
mod domain;
mod device;
#[cfg(test)]
mod test;
use conf::Conf;
use debugging::session::debug_session::{DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::ThreadPool;
use server::Server;

use crate::server::{Content, Cot, DevStream, QueryId, SelectAct, SelectContent, SelectCot, SelectDevDoc, SelectDevInfo, SelectReq};

///
/// Application entry point
fn main() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let dbg = Dbg::own("idm-server");

    let tp = ThreadPool::new(&dbg, Some(12));
    match Conf::load("config.yaml") {
        Ok(conf) => {
            let server = Server::new(
                &dbg,
                conf.clone(),
                tp.scheduler(),
                move |dbg, conf| { Box::new(
                    //
                    // Select handler for incomong messages by Content
                    SelectContent::new(vec![
                        // Handler for Content::Bytes
                        (Content::Bytes, Box::new(SelectCot::new(vec![
                            // Handling incomong messages with `Cot::Act` by field `cmd`
                            (Cot::Act, Box::new(SelectAct::new(vec![
                                // Handling incomong commands
                                // ...
                            ]))),
                            // Handling incomong messages with Cot::Req by field `req`
                            (Cot::Req, Box::new(SelectReq::new(vec![
                                // Handling incomong requests
                                // ...
                            ]))),
                        ]))),
                        // Handler for Content::Empty
                        (Content::Empty, Box::new(SelectCot::new(vec![
                            // Handling incomong messages with `Cot::Act` by field `cmd`
                            (Cot::Act, Box::new(SelectAct::new(vec![
                                // Handling incomong commands
                                // ...
                            ]))),
                            // Handling incomong messages with Cot::Req by field `req`
                            (Cot::Req, Box::new(SelectReq::new(vec![
                                // Handling incomong requests
                                // ...
                            ]))),
                        ]))),
                        // Handler for Content::Json
                        (Content::Json, Box::new(SelectCot::new(vec![
                            // Handling incomong messages with `Cot::Act` by field `cmd`
                            (Cot::Act, Box::new(SelectAct::new(
                                vec![
                                    // Handling incomong command `DeviceStream`
                                    (QueryId::DeviceStream, Box::new(DevStream::new(
                                        dbg,
                                        conf.server.dev_stream.clone(),
                                        tp.scheduler(),
                                    ))),
                                ]
                            ))),
                            // Handling incomong messages with Cot::Req by field `req`
                        (Cot::Req, Box::new(SelectReq::new(vec![
                                // Handling incomong request `DeviceInfo`
                                (QueryId::DeviceInfo, Box::new(SelectDevInfo::new("assets/info/"))),
                                // Handling incomong request `DeviceDoc`
                                (QueryId::DeviceDoc, Box::new(SelectDevDoc::new("assets/info/"))),
                            ]))),
                        ]))),
                    ])
                )}
            );
            if let Err(err) = server.run() {
                log::warn!("{dbg} | Error: {:?}", err);
            }
            if let Err(err) = server.wait() {
                log::warn!("{dbg} | Error: {:?}", err);
            }
        }
        Err(err) => {
            log::warn!("{dbg} | Error: {:?}", err);
        }
    }
}
