use std::path::{Path, PathBuf};
use crate::{device::{DevId, DeviceDoc}, domain::{Error, EvalEx}, server::{EvalResult, Query, QueryId, Reply, Request}};

///
/// Extracting incoming messages as [DeviceDocRequest]
/// - Forwarding requested id to the specified `ctx`
/// - Returns [DeviceDoc]
pub struct SelectDevDoc {
    path: PathBuf,
}
//
//
impl SelectDevDoc {
    ///
    /// Returns [SelectDevDoc] new instance
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }
}
//
//
impl EvalEx<Request<QueryId, Query>, EvalResult> for SelectDevDoc {
    //
    fn eval(&self, req: Request<QueryId, Query>) -> EvalResult {
        let error = Error::new("SelectDevDoc", "eval");
        match &req.query {
            Query::DeviceDoc(query) => {
                match DeviceDoc::from_path("assets/info/").eval(DevId(query.dev_id.clone())) {
                    Ok(data) => Ok(Some(req.reply(Reply::DeviceDoc(data)))),
                    Err(err) => Err(error.pass(err.to_string())),
                }
            }
            _ => Err(error.err(format!("Query::DeviceDoc expected, but found {:?}", req.query_id))),
        }
    }
    //
    fn exit(&self) {
        // Halt continuous operations here
    }
}
//
//
unsafe impl Send for SelectDevDoc {}
