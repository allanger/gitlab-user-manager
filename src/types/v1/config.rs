use serde::{Deserialize, Serialize};

use super::{team::Team, user::User};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)] 
    pub(crate) teams: Vec<Team>,
    #[serde(default)] 
    pub(crate) users: Vec<User>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            teams: vec![Team {
                name: "default".to_string(),
                ..Default::default()
            }],
            users: Default::default(),
        }
    }
}
