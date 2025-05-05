use std::path::Path;

use sal_core::error::Error;

use crate::server::server_conf::ServerConf;

///
/// The application configuration
pub struct Conf {
    server: ServerConf,
    api: ApiConf,
}
//
//
impl Conf {
    ///
    /// Returns [Cong] new instance loaded from yaml
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self {
            
        })
    }
}