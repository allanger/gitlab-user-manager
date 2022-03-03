use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Ownership {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) url: String,
}
