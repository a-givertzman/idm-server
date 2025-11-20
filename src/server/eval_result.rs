use sal_core::error::Error;
use crate::server::Response;

pub type EvalResult = Result<Option<Response>, Error>;