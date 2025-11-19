use std::{fs::OpenOptions, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use crate::{device::DevId, domain::{Error, EvalEx}};
///
/// Reply to `DeviceDoc` request
/// - Provides basic overview info by device
/// 
///
/// Creates a new instanse of [DeviceDoc] with fields:
/// - `manufacturer` manufacturing company
/// - `vendor` - company destributed equipment
/// - `order_code` - equipment order number
/// - `model` - equipment model name
/// - `serial` - equipment serial number
/// - `name` - equipment name
/// - `description` - detailed description of the equipment
/// - `width` - equipmets width
/// - `height` - equipment height
/// - `depth` -  equipment depth
/// - `weight` - equipment weight
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceDoc {
    #[serde(skip)]
    path: PathBuf,
    pub id: String,
    pub manufacturer: String,
    pub vendor: String,
    #[serde(rename="order-code")]
    pub order_code: String,
    pub model: String,
    pub serial: String,
    pub name: String,
    pub description: String,
    pub width: String,
    pub height: String,
    pub depth: String,
    pub weight: String,
}
//
//
impl DeviceDoc {
    ///
    /// Returns [DeviceDoc] ready to be read using `eval` method from the specified `path` and passed `id`
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            id: Default::default(),
            manufacturer: Default::default(),
            vendor: Default::default(),
            order_code: Default::default(),
            model: Default::default(),
            serial: Default::default(),
            name: Default::default(),
            description: Default::default(),
            width: Default::default(),
            height: Default::default(),
            depth: Default::default(),
            weight: Default::default() 
        }
    }
    ///
    /// Returns [DeviceDoc] read from path 
    fn read<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let error = Error::new("DeviceDoc", "read");
        let file = OpenOptions::new()
            .read(true)
            .open(path);
        match file {
            Ok(file) => serde_json::from_reader(file).map_err(|err| error.pass(err.to_string())),
            Err(err) => Err(error.pass(err.to_string())),
        }
    }
}
//
//
impl EvalEx<DevId, Result<Self, Error>> for DeviceDoc {
    //
    fn eval(&self, id: DevId) -> Result<Self, Error> {
        let error = Error::new("DeviceDoc", "eval");
        let path = self.path.join(format!("{}.md", id.0));
        match Self::read(path) {
            Ok(value) => Ok(value),
            Err(err) => Err(error.pass(err)),
        }
    }
    //
    //
    fn exit(&self) {
        // Halt continuous operations here
    }
}
