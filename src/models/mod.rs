pub(crate) mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]

    pub struct Config {
        pub(crate) teams: Option<Vec<Team>>,
        pub(crate) users: Option<Vec<User>>,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct User {
        pub(crate) id: i32,
        pub(crate) name: String,
        pub(crate) teams: Vec<String>,
        pub(crate) projects: Vec<Project>,
        pub(crate) ownerships: Vec<Ownership>,
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
        pub(crate) id: i32,
        pub(crate) access_right: String,
    }
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Team {
        pub(crate) name: String,
        pub(crate) projects: Option<Vec<Project>>,
    }
}
