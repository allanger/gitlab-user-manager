use serde::{Deserialize, Serialize};

use super::{namespace::Namespace, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Team {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) projects: Vec<Project>,
    #[serde(default)]
    pub(crate) namespaces: Vec<Namespace>,
}
