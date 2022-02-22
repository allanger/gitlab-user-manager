use serde::{Deserialize, Serialize};

use super::{ownership::Ownership, project::Project};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct State {
    pub(crate) user_id: u64,
    pub(crate) projects: Vec<Project>,
    pub(crate) ownerships: Vec<Ownership>,
}
