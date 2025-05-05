use std::{fs::OpenOptions, path::Path};
use sal_core::error::Error;
use serde::{Deserialize, Serialize};
use crate::{api::api_conf::ApiConf, server::server_conf::ServerConf};

///
/// The application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    pub server: ServerConf,
    pub api: ApiConf,
}
//
//
impl Conf {
    ///
    /// Returns [Cong] new instance loaded from yaml
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let error = Error::new("Conf", "load");
        let file = OpenOptions::new()
            .read(true)
            .open(path);
        match file {
            Ok(file) => {
                match serde_yaml::from_reader(file) {
                    Ok(value) => {
                        let conf: Conf = value;
                        Ok(conf)
                    }
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
}