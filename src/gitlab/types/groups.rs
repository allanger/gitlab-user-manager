use serde::Deserialize;
use tabled::Tabled;

use crate::{gitlab::CustomMember, types::v1::{namespace::Namespace, access_level::AccessLevel}};
use std::io::Result;

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
