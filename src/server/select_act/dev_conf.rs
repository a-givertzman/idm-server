use std::time::Duration;
use serde::{Deserialize, Deserializer, Serialize};

///
/// The `DevStream`'s configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConf {
    pub value: f64,
    pub deviation: f64,
    #[serde(deserialize_with = "DevConf::interval")]
    pub interval: Duration,
}
//
//
impl DevConf {
    const DEFAULT_INTERVAL: Duration = Duration::from_millis(100);
    ///
    /// Used to deserialize `interval`
    fn interval<'de, D>(deserializer: D) -> Result<Duration, D::Error> where D: Deserializer<'de> {
        match Deserialize::deserialize(deserializer) {
            Ok(val) => {
                let val: Option<u64> = val;
                match val {
                    Some(val) => Ok(Duration::from_millis(val)),
                    None => {
                        log::info!("DevConf.interval | Field `interval` is missing, used default {:?}", Self::DEFAULT_INTERVAL);
                        Ok(Self::DEFAULT_INTERVAL)
                    }
                }
            },
            Err(err) => {
                log::info!("DevConf.interval | Field `interval` deserialize error {:?}, used default {:?}", err, Self::DEFAULT_INTERVAL);
                Ok(Self::DEFAULT_INTERVAL)
            }
        }
    }
}
