use serde::{Deserialize, Serialize};

use super::access_level::AccessLevel;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Namespace {
    pub(crate) name: String,
    pub(crate) access_level: AccessLevel,
    pub(crate) id: u64,
    pub(crate) url: String,
}
