use serde::Deserialize;
use tabled::Tabled;

use crate::{
    gitlab::CustomMember,
    types::v1::{access_level::AccessLevel, project},
};
use std::io::Result;

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl Project {
    pub(crate) fn to_gum_project(&self, member: CustomMember) -> Result<project::Project> {
        let project = project::Project {
            id: self.id,
            name: self.name.clone(),
            access_level: AccessLevel::from_gitlab_access_level(member.access_level),
        };
        Ok(project)
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct ProjectsWithShared {
    shared_with_groups: Vec<SharedWithGroups>,
}

impl ProjectsWithShared {
    /// Get a reference to the groups with shared's shared with groups.
    pub(crate) fn shared_with_groups(&self) -> Vec<SharedWithGroups> {
        self.shared_with_groups.clone()
    }
}
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SharedWithGroups {
    pub(crate) group_id: u64,
    pub(crate) group_name: String,
    pub(crate) group_access_level: gitlab::AccessLevel,
}
