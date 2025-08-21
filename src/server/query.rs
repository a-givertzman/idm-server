use sal_sync::services::entity::Cot;
use serde::{Deserialize, Serialize};
use crate::{domain::{JsonMap, JsonVal}};

///
/// The [Query] contains the name and data bytes to be sent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query<R> {
    /// Query id, used internal only to identify incoming request message
    #[serde(skip)]
    pub msg_id: usize,
    /// Name of the [Query]
    pub name: R,
    /// cause of the transmission
    pub cot: Cot,
    /// Payload data of the [Query]
    pub content: JsonMap<String, JsonVal>,
}
//
//
impl<R> Query<R> {
    ///
    /// Returns [Query] new instance
    /// - `id` - Query id, used internal only to identify incoming request message
    /// - `name` - Name of the [Query]
    /// - `bytes` - 
    pub fn new(id: usize, name: R, cot: Cot, content: JsonMap<String, JsonVal>) -> Self {
        Self {
            msg_id: id,
            name,
            cot,
            content,
        }
    }
}
