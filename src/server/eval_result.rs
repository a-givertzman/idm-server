use sal_core::error::Error;
use crate::domain::JsonVal;

pub type EvalResult = Result<Option<JsonVal>, Error>;