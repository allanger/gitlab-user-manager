use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Meta {
    version: String,
    #[serde(default)]
    groups: Vec<Group>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
            ..Default::default()
        }
    }
}
