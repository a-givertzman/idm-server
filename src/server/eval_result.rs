use sal_core::error::Error;
use crate::server::{QueryId, Reply, Response};

pub type EvalResult = Result<Option<Response<QueryId, Reply>>, Error>;