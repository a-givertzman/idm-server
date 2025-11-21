use std::path::{Path, PathBuf};
use crate::{device::{DevId, DeviceDoc}, domain::{Error, EvalEx}, server::{EvalResult, Query, Reply, Request, extract}};

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
impl EvalEx<Request, EvalResult> for SelectDevDoc {
    //
    fn eval(&self, req: Request) -> EvalResult {
        let error = Error::new("SelectDevDoc", "eval");
        let query = extract!(&req.query, Query::DeviceInfo)
            .map_err(|_| error.err(format!("Query::DeviceDoc expected, but found {:?}", req.query_id)))?;
        match DeviceDoc::from_path(&self.path).eval(DevId(query.dev_id.clone())) {
            Ok(data) => Ok(Some(req.reply(Reply::DeviceDoc(data)))),
            Err(err) => Err(error.pass(err.to_string())),
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
unsafe impl Send for SelectDevDoc {}
