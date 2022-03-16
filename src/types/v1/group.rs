use serde::{Deserialize, Serialize};

use super::{namespace::Namespace, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Group {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) projects: Vec<Project>,
    pub(crate) namespaces: Vec<Namespace>,
}
