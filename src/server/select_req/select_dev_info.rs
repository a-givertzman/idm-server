use std::path::{Path, PathBuf};
use crate::{device::{DevId, DeviceInfo}, domain::{Error, EvalEx}, server::{EvalResult, Query, Reply, Request, extract}};

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
impl EvalEx<Request, EvalResult> for SelectDevInfo {
    //
    fn eval(&self, req: Request) -> EvalResult {
        let error = Error::new("SelectDevInfo", "eval");
        let query = extract!(&req.query, Query::DeviceInfo)
            .map_err(|_| error.err(format!("Query::DeviceInfo expected, but found {:?}", req.query_id)))?;
        match DeviceInfo::from_path(&self.path).eval(DevId(query.dev_id.clone())) {
            Ok(data) => Ok(Some(req.reply(Reply::DeviceInfo(data)))),
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
