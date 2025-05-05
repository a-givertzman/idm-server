mod api;
mod conf;
mod server;
mod types;
use conf::conf::Conf;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use server::server::Server;


///
/// Application entry point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = Dbg::own("idm-server");

    match Conf::load("config.yaml") {
        Ok(conf) => {
            let server = Server::new(conf.server);
            if let Err(err) = server.run() {
                log::warn!("{dbg} | Error: {:?}", err);
            }
        }
        Err(err) => {
            log::warn!("{dbg} | Error: {:?}", err);
        }
    }
}
