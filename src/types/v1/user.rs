use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{namespace::Namespace, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) teams: Vec<String>,
    pub(crate) projects: Vec<Project>,
    pub(crate) namespaces: Vec<Namespace>,
}

impl User {
    pub(crate) fn to_hashmap(&self) -> HashMap<u64, Self> {
        let mut user_map: HashMap<u64, User> = HashMap::new();
        user_map.insert(self.id, self.clone());
        user_map
    }
}
