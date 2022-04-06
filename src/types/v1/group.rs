use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{namespace::Namespace, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Group {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) projects: Vec<Project>,
    pub(crate) namespaces: Vec<Namespace>,
}

impl Group {
    // Transform a vector of Groups to HashMap<group_id, Group>
    pub(crate) fn to_hashmap(&self) -> HashMap<u64, Self> {
        let mut map: HashMap<u64, Group> = HashMap::new();
        map.insert(self.id, self.clone());
        map
    }
}
