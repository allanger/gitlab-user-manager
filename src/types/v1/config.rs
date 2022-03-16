use serde::{Deserialize, Serialize};

use super::{group::Group, team::Team, user::User};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub(crate) teams: Vec<Team>,
    #[serde(default)]
    pub(crate) users: Vec<User>,
    #[serde(default)]
    pub(crate) groups: Vec<Group>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            teams: vec![Team {
                name: "default".to_string(),
                ..Default::default()
            }],
            users: Default::default(),
            groups: Default::default(),
        }
    }
}
