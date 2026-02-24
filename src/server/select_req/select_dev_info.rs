use std::{fmt::Debug, path::{Path, PathBuf}};
use crate::{device::{DevId, DeviceInfo}, domain::{Error, EvalEx}, server::{EvalResult, Query, Reply, Frame, extract}};

///
/// Extracting incoming messages as [DeviceInfoRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceInfo]
pub(crate) struct SelectDevInfo {
    path: PathBuf,
}
//
//
impl SelectDevInfo {
    ///
    /// Returns [SortByX] new instance
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}
//
//
impl<K: Debug + Copy> EvalEx<Frame<K>, EvalResult<K>> for SelectDevInfo {
    //
    fn eval(&self, frame: Frame<K>) -> EvalResult<K> {
        let error = Error::new("SelectDevInfo", "eval");
        let query = frame.operation().map_err(|err| error.pass(err))?;
        let query = extract!(&query, Query::DeviceInfo)
            .map_err(|_| error.err(format!("Query::DeviceInfo expected, but found {:?}", frame.operation_id)))?;
        match DeviceInfo::from_path(&self.path).eval(DevId(query.dev_id.clone())) {
            Ok(data) => Ok(Some(frame.reply(Reply::DeviceInfo(data)))),
            Err(err) => Err(error.pass(err)),
        }
    }
    ///
    /// Halts hanbler
    fn exit(&self) {
        // Halt continuous operations here
    }
}
//
//
unsafe impl Send for SelectDevInfo {}
