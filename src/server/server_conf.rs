use serde::{Deserialize, Serialize};
use super::ConnectionConf;

///
/// The `Server` configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConf {
    pub address: String,
    pub connection: ConnectionConf,
}
