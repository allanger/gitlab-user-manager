use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Ownership {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) url: String,
}

impl Default for Ownership {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: u64::MIN,
            url: String::new(),
        }
    }
}
