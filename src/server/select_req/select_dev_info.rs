use std::path::{Path, PathBuf};
use crate::{device::{DevId, DeviceInfo}, domain::{Error, EvalEx}, server::{EvalResult, Query, QueryId, Reply, Request}};

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
impl EvalEx<Request<QueryId, Query>, EvalResult> for SelectDevInfo {
    //
    fn eval(&self, req: Request<QueryId, Query>) -> EvalResult {
        let error = Error::new("SelectDevInfo", "eval");
        match &req.query {
            Query::DeviceInfo(query) => {
                match DeviceInfo::from_path("assets/info/").eval(DevId(query.dev_id.clone())) {
                    Ok(data) => Ok(Some(req.reply(Reply::DeviceInfo(data)))),
                    Err(err) => Err(error.pass(err)),
                }
            }
            _ => Err(error.err(format!("Query::DeviceInfo expected, but found {:?}", req.query_id))),
        }
    }
    //
    fn exit(&self) {
        // Halt continuous operations here
    }
}
//
//
unsafe impl Send for SelectDevInfo {}
