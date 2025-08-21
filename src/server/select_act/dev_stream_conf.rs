use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use super::DevConf;

///
/// The `DevStream`'s configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevStreamConf {
    /// TCP socket read/write timeout in ms, default 100 ms
    // #[serde(deserialize_with = "DevStreamConf::timeout")]
    pub devices: IndexMap<String, DevConf>,
}
//
//
// impl DevStreamConf {
//     const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);
//     ///
//     /// Used to deserialize `timeout`
//     fn timeout<'de, D>(deserializer: D) -> Result<Duration, D::Error> where D: Deserializer<'de> {
//         match Deserialize::deserialize(deserializer) {
//             Ok(val) => {
//                 let val: Option<u64> = val;
//                 match val {
//                     Some(val) => Ok(Duration::from_millis(val)),
//                     None => {
//                         log::info!("DevStreamConf.timeout | Field `timeout` is missing, used default {:?}", Self::DEFAULT_TIMEOUT);
//                         Ok(Self::DEFAULT_TIMEOUT)
//                     }
//                 }
//             },
//             Err(err) => {
//                 log::info!("DevStreamConf.timeout | Field `timeout` deserialize error {:?}, used default {:?}", err, Self::DEFAULT_TIMEOUT);
//                 Ok(Self::DEFAULT_TIMEOUT)
//             }
//         }
//     }
// }