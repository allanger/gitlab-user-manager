use gitlab::{
    api::{groups, Query},
    Gitlab,
};
use serde::Deserialize;
use std::io::{Error, Result};

use crate::output::out_message::OutMessage;

#[derive(Debug, Deserialize, Clone, Default)]
struct GroupsWithShared {
    shared_with_groups: Vec<SharedWithGroups>,
}

impl GroupsWithShared {
    /// Get a reference to the groups with shared's shared with groups.
    fn shared_with_groups(&self) -> Vec<SharedWithGroups> {
        self.shared_with_groups.clone()
    }
}
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct SharedWithGroups {
    pub(crate) group_id: u64,
    pub(crate) group_name: String,
    pub(crate) group_access_level: gitlab::AccessLevel,
}

impl SharedWithGroups {
    pub(crate) fn get(group_id: u64, gitlab_client: Gitlab) -> Result<Vec<SharedWithGroups>> {
        let group = match groups::Group::builder().group(group_id).build() {
            Ok(group) => group,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let shared: GroupsWithShared = group.query(&gitlab_client).unwrap_or_else(|err| {
            OutMessage::message_info_clean(format!("{}", err).as_str());

            return GroupsWithShared::default();
        });
        let r = shared.shared_with_groups();
        Ok(r)
    }
}
