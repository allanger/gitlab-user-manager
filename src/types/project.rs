use serde::{Deserialize, Serialize};

use super::access_level::AccessLevel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) access_level: AccessLevel,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: u64::MIN,
            access_level: Default::default(),
        }
    }
}
