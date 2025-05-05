use serde::{Deserialize, Serialize};

///
/// The `ServerConnection` configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConnectionConf {
    pub address: String,
}
