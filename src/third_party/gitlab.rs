use std::io::Error;

use gitlab::{
    api::{groups, projects, users, Query},
    Gitlab,
};
use serde::Deserialize;

pub(crate) struct GitlabClient {
    gitlab_client: Gitlab,
}

// TODO: Get rid of
pub struct GitlabConnection {
    pub url: String,
    pub token: String,
}

pub(crate) fn new_gitlab_client_deprecated(url: String, token: String) -> impl GitlabActions {
    GitlabClient {
        gitlab_client: Gitlab::new(url, token).unwrap(),
    }
}

pub(crate) fn new_gitlab_client(client: Gitlab) -> impl GitlabActions {
    GitlabClient {
        gitlab_client: client,
    }
}

pub(crate) trait GitlabActions {
    fn get_project_data_by_id(&self, id: u64) -> Result<Project, Error>;
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error>;
    fn get_group_data_by_id(&self, id: u64) -> Result<Group, Error>;
}

#[derive(Debug, Deserialize)]
pub(crate) struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
}
#[derive(Debug, Deserialize)]

pub(crate) struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl GitlabActions for GitlabClient {
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
    fn get_group_data_by_id(&self, id: u64) -> Result<Group, Error> {
        let group = match groups::Group::builder().group(id).build() {
            Ok(group) => group,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let output: Group = group.query(&self.gitlab_client).unwrap();
        Ok(output)
    }
}
