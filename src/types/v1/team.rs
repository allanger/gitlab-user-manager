use serde::{Deserialize, Serialize};

use super::project::Project;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Team {
    pub(crate) name: String,
    pub(crate) projects: Vec<Project>,
}
