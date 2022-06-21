use crate::{
    gitlab::CustomMember,
    types::v1::{AccessLevel, Namespace},
};
use serde::Deserialize;
use std::io::Result;
use tabled::Tabled;

// Simple group struct
#[derive(Debug, Deserialize, Tabled, Clone)]
pub(crate) struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl Group {
    pub(crate) fn to_gum_group(&self, member: CustomMember) -> Result<Namespace> {
        let group = Namespace {
            id: self.id,
            name: self.name.clone(),
            url: self.web_url.clone(),
            access_level: AccessLevel::from_gitlab_access_level(member.access_level),
        };
        Ok(group)
    }
}

// Struct for parsing groups with which the current one is shared
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SharedWithGroups {
    pub(crate) group_id: u64,
    pub(crate) group_name: String,
    pub(crate) group_access_level: gitlab::AccessLevel,
}

// TODO: Get rid of this struct
#[derive(Debug, Deserialize, Clone, Default)]
pub(crate) struct GroupsWithShared {
    shared_with_groups: Vec<SharedWithGroups>,
}

impl GroupsWithShared {
    /// Get a reference to the groups with shared's shared with groups.
    pub(crate) fn shared_with_groups(&self) -> Vec<SharedWithGroups> {
        self.shared_with_groups.clone()
    }
}
