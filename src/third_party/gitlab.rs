use std::io::Error;

use gitlab::{
    api::{projects, users, Query},
    Gitlab,
};
use serde::Deserialize;

pub(crate) struct RsGitlab {
    gitlab_client: Gitlab,
}

pub struct GitlabConnection {
    pub url: String,
    pub token: String,
}

pub(crate) fn new_gitlab_client(url: String, token: String) -> impl GitlabActions {
    RsGitlab {
        gitlab_client: Gitlab::new(url, token).unwrap(),
    }
}

pub(crate) trait GitlabActions {
    fn get_project_data_by_id(&self, id: u64) -> Result<Project, Error>;
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error>;
}

#[derive(Debug, Deserialize)]
pub(crate) struct Project {
    pub(crate) name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct User {
    pub(crate) name: String,
}

impl GitlabActions for RsGitlab {
    fn get_project_data_by_id(&self, id: u64) -> Result<Project, Error> {
        let project = match projects::Project::builder().project(id).build() {
            Ok(project) => project,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let output: Project = project.query(&self.gitlab_client).unwrap();
        Ok(output)
    }
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error> {
        let user = match users::User::builder().user(id).build() {
            Ok(user) => user,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let output: User = user.query(&self.gitlab_client).unwrap();
        Ok(output)
    }
}
