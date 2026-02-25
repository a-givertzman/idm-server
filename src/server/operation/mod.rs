mod operation_id;
mod query;

pub(crate) use operation_id::*;
pub(crate) use query::*;

/// Configuration for the binary encoding / decoding with default parameters
/// - Little endian
/// - Variable int encoding
pub const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();
