use sal_core::error::Error;

use super::server_conf::ServerConf;

pub struct Server {

}
//
//
impl Server {
    ///
    /// Returns [Server] new instance
    pub fn new(conf: ServerConf) -> Self {
        Self {
            
        }
    }
    ///
    /// [Server] Operation mode
    pub fn run(&self) -> Result<(), Error> {
        Ok(())
    }
}