use std::{fs::OpenOptions, path::{Path, PathBuf}};
use sal_core::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{domain::Eval, server::JsonCtx};
///
/// Wrapper for the [DeviceInfo] id of type u32
pub struct DevId(pub u32);
///
/// Reply to `DeviceInfo` request
/// - Provides basic overview info by device
/// 
///
/// Creates a new instanse of [DeviceInfo] with fields:
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
pub struct DeviceInfo {
    #[serde(skip)]
    path: PathBuf,
    pub id: usize,
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
impl DeviceInfo {
    ///
    /// Returns [DeviceInfo] ready to be read using `eval` method from the specified `path` and passed `id`
    pub fn new(
        id: usize,
        manufacturer: String,
        vendor: String,
        order_code: String,
        model: String,
        serial: String,
        name: String,
        description: String,
        width: String,
        height: String,
        depth: String,
        weight: String,
    ) -> Self {
        Self {
            path: PathBuf::new(),
            id,
            manufacturer,
            vendor,
            order_code,
            model,
            serial,
            name,
            description,
            width,
            height,
            depth,
            weight,
        }
    }
    ///
    /// Returns [DeviceInfo] ready to be read using `eval` method from the specified `path` and passed `id`
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
    /// Returns [DeviceInfo] read from path 
    fn read<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .open(path);
        match file {
            Ok(file) => {
                match serde_json::from_reader(file) {
                    Ok(value) => {
                        let result: Self = value;
                        Ok(result)
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }
}
//
//
impl Eval<DevId, Result<JsonCtx, Error>> for DeviceInfo {
    fn eval(&mut self, id: DevId) -> Result<JsonCtx, Error> {
        let error = Error::new("DeviceInfo", "eval");
        let path = self.path.join(format!("{}.json", id.0));
        match Self::read(path) {
            Ok(value) => Ok(JsonCtx { id, value: json!(value) }),
            Err(err) => Err(error.pass(err)),
        }
    }
}
