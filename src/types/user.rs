use serde::{Deserialize, Serialize};

use super::{ownership::Ownership, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) teams: Vec<String>,
    pub(crate) projects: Vec<Project>,
    pub(crate) ownerships: Vec<Ownership>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: u64::MIN,
            name: String::new(),
            teams: Vec::new(),
            projects: Vec::new(),
            ownerships: Vec::new(),
        }
    }
}
