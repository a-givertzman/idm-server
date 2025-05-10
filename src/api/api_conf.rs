use serde::{Deserialize, Serialize};

///
/// The `Server` configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConf {
    pub address: String,
}
