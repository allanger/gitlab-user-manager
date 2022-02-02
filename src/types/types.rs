use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub(crate) teams: Option<Vec<Team>>,
    pub(crate) users: Option<Vec<User>>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) teams: Option<Vec<String>>,
    pub(crate) projects: Option<Vec<Project>>,
    pub(crate) ownerships: Option<Vec<Ownership>>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ownership {
    pub(crate) name: String,
    pub(crate) id: i32,
    pub(crate) url: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) access_right: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub(crate) name: String,
    pub(crate) projects: Option<Vec<Project>>,
}
