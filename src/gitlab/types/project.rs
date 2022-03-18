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
