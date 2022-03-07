use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Meta {
    version: String,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
        }
    }
}
