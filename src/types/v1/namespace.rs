use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::access_level::AccessLevel;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Namespace {
    pub(crate) name: String,
    pub(crate) access_level: AccessLevel,
    pub(crate) id: u64,
    pub(crate) url: String,
}

impl Namespace {
    pub(crate) fn to_hashmap(&self) -> HashMap<u64, Self> {
        let mut namespace_map: HashMap<u64, Namespace> = HashMap::new();
        namespace_map.insert(self.id, self.clone());
        namespace_map

    }
}
