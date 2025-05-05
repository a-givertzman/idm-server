use serde::{Deserialize, Serialize};

///
/// The `Server` configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConf {
    pub address: String,
}
