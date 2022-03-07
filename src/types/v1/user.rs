use serde::{Deserialize, Serialize};

use super::{group::Group, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) teams: Vec<String>,
    pub(crate) projects: Vec<Project>,
    pub(crate) groups: Vec<Group>,
}
