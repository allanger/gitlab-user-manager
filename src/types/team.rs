use serde::{Deserialize, Serialize};

use super::project::Project;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Team {
    pub(crate) name: String,
    pub(crate) projects: Vec<Project>,
}

impl Default for Team {
    fn default() -> Self {
        Self {
            name: String::new(),
            projects: Vec::new(),
        }
    }
}
