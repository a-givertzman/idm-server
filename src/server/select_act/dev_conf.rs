use serde::{Deserialize, Serialize};

///
/// The `DevStream`'s configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConf {
    pub value: f64,
    pub deviation: f64,
}