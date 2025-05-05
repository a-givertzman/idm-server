mod api;
mod conf;
mod server;
mod types;
use conf::Conf;
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use sal_core::dbg::Dbg;
use sal_sync::thread_pool::tread_pool::ThreadPool;
use server::Server;


///
/// Application entry point
fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    let dbg = Dbg::own("idm-server");

    let thread_pool = ThreadPool::new(&dbg, Some(12));
    match Conf::load("config.yaml") {
        Ok(conf) => {
            let server = Server::new(&dbg, conf.server, thread_pool.scheduler());
            if let Err(err) = server.run() {
                log::warn!("{dbg} | Error: {:?}", err);
            }
        }
        Err(err) => {
            log::warn!("{dbg} | Error: {:?}", err);
        }
    }
}
