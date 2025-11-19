use std::time::{Duration, Instant};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::device::DevConf;

///
/// Device stream info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    /// Like On/Off
    pub state: String,
    /// Like speed, current, etc...
    pub value: String,
    pub conf: DevConf,
    #[serde(skip)]
    pub val: f64,
    #[serde(skip)]
    interval: Option<Instant>,
}
impl Device {
    ///
    /// Returns [Device] new instance
    pub fn new(id: String, state: String, val: f64, conf: DevConf) -> Self {
        Self {
            id,
            state,
            value: format!("{:.3}", val),
            conf,
            val,
            interval: Some(Instant::now()),
        }
    }
    ///
    /// Elapsed from las send of [Device] 
    pub fn elapsed(&self) -> Duration {
        match self.interval {
            Some(interval) => interval.elapsed(),
            None => panic!("Device({}).elapsed | Interval is not initialized", self.id),
        }
    }
}
//
//
impl From<&Device> for Device {
    ///
    /// Returns [Device] created from it prevouse state
    /// using configured `conf.diviation`
    fn from(dev: &Device) -> Self {
        let mut rng = rand::rng();
        let sign = rng.random::<bool>();
        let deviation = dev.conf.deviation * 0.1 * rng.random::<f64>();
        let val = match sign {
            true => dev.val + deviation,
            false => dev.val - deviation,
        };
        Self {
            id: dev.id.to_owned(),
            state: dev.state.to_owned(),
            value: format!("{:.3}", val),
            conf: dev.conf.to_owned(),
            val,
            interval: Some(Instant::now()),
        }
    }
}