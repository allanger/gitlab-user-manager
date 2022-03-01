use core::time;
use std::{
    io::{Error, ErrorKind},
    thread,
};

use gitlab::{
    api::{self, groups, projects, users, ApiError, Query},
    Gitlab,
};
use serde::Deserialize;
use tabled::Tabled;

use crate::{
    output::{OutMessage, OutSpinner},
    types::access_level::AccessLevel,
};

pub(crate) struct GitlabClient {
    gitlab_client: Gitlab,
}

impl GitlabClient {
    pub(crate) fn new(client: Gitlab) -> Self {
        Self {
            gitlab_client: client,
        }
    }
}

pub(crate) trait GitlabActions {
    fn get_project_data_by_id(&self, id: u64) -> Result<Project, Error>;
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error>;
    fn get_group_data_by_id(&self, id: u64) -> Result<Group, Error>;
    fn add_user_to_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
    fn add_user_to_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<String, Error>;
    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<String, Error>;
    fn edit_user_in_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
    fn edit_user_in_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
}

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) web_url: String,
}
#[derive(Debug, Deserialize, Tabled)]

pub(crate) struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl GitlabActions for GitlabClient {
    fn get_project_data_by_id(&self, id: u64) -> Result<Project, Error> {
        let project = match projects::Project::builder().project(id).build() {
            Ok(project) => project,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };

        let output: Project = match project.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { ref obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                OutMessage::message_info_with_alias("Gitlab is screwed by amount of your requests. You need to wait");
                                let spinner = OutSpinner::spinner_start("Waiting 30s".to_string());
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                spinner.spinner_success("Let's try again".to_string());
                                return self.get_project_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, err));
            }
            Ok(res) => res,
        };
        Ok(output)
    }
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error> {
        let user = match users::User::builder().user(id).build() {
            Ok(user) => user,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let output: User = match user.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { ref obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                            OutMessage::message_info_with_alias("Gitlab is screwed by amount of your requests. You need to wait");
                            let spinner = OutSpinner::spinner_start("Waiting 30s".to_string());
                            let await_time = time::Duration::from_secs(30);
                            thread::sleep(await_time);
                            spinner.spinner_success("Let's try again".to_string());
                            return self.get_user_data_by_id(id);
                        };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, err));
            }
            Ok(res) => res,
        };

        Ok(output)
    }
    fn get_group_data_by_id(&self, id: u64) -> Result<Group, Error> {
        let group = match groups::Group::builder().group(id).build() {
            Ok(group) => group,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let output: Group = match group.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { ref obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                            OutMessage::message_info_with_alias("Gitlab is screwed by amount of your requests. You need to wait");
                            let spinner = OutSpinner::spinner_start("Waiting 30s".to_string());
                            let await_time = time::Duration::from_secs(30);
                            thread::sleep(await_time);
                            spinner.spinner_success("Let's try again".to_string());
                            return self.get_group_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, err));
            }
            Ok(res) => res,
        };
        Ok(output)
    }

    fn add_user_to_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
        let q = match projects::members::AddProjectMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .project(pid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Added".to_string()),
            Err(err) => {
                if let ApiError::Gitlab { msg } = err {
                    if msg == "Member already exists" {
                        return Ok("Already added".to_string());
                    }
                    return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                } else {
                    return Err(Error::new(ErrorKind::AddrNotAvailable, err));
                };
            }
        };
    }

    fn add_user_to_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
        let q = match groups::members::AddGroupMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Added".to_string()),
            Err(err) => {
                if let ApiError::Gitlab { msg } = err {
                    if msg == "Member already exists" {
                        return Ok("Already exists".to_string());
                    }
                    return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                } else {
                    return Err(Error::new(ErrorKind::AddrNotAvailable, err));
                };
            }
        };
    }

    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<String, Error> {
        let q = match projects::members::RemoveProjectMember::builder()
            .user(uid)
            .project(pid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Removed".to_string()),
            Err(err) => {
                match err {
                    ApiError::Gitlab { msg } => {
                        if msg == "404 Not found" {
                            return Ok("Not found".to_string());
                        }
                        return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
            }
        };
    }

    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<String, Error> {
        let q = match groups::members::RemoveGroupMember::builder()
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Removed".to_string()),
            Err(err) => {
                match err {
                    ApiError::Gitlab { msg } => {
                        if msg == "404 Not found" {
                            return Ok("Not found".to_string());
                        }
                        return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
            }
        };
    }

    fn edit_user_in_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
        let q = match projects::members::EditProjectMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .project(pid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Updated".to_string()),
            Err(_) => return Err(Error::new(ErrorKind::AddrNotAvailable, "asd")),
        };
    }

    fn edit_user_in_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
        let q = match groups::members::EditGroupMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok("Updated".to_string()),
            Err(_) => return Err(Error::new(ErrorKind::AddrNotAvailable, "asd")),
        };
    }
}
