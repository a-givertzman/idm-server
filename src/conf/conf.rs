use std::{fs::OpenOptions, path::Path};
use serde::{Deserialize, Serialize};
use crate::{domain::Error, server::ServerConf};

///
/// The application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
    pub server: ServerConf,
}
//
//
impl Conf {
    ///
    /// Returns [Conf] new instance loaded from yaml
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