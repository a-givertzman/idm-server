use serde::{Deserialize, Serialize};

///
/// Cose of transmission
/// - `Act` - Activation
/// - `ActCon` - Activation confirmation
/// - `Inf` - Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cot {
    Act,
    ActCon,
    Inf,
}