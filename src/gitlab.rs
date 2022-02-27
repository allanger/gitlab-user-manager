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

use crate::types::access_level::AccessLevel;

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
    fn add_user_to_group(&self, uid: u64, gid: u64, access_level: AccessLevel)
        -> Result<(), Error>;
    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<(), Error>;
    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<(), Error>;
    fn edit_user_in_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<(), Error>;
    fn edit_user_in_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<(), Error>;
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
        let output: Project = match project.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                println!("Gitlab is screw by amount of our requests. I'm sorry, buy you need to wait, mate");
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                return self.get_project_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, "asd"));
            }
            Ok(res) => res,
        };
        Ok(output)
    }
    fn get_user_data_by_id(&self, id: u64) -> Result<User, Error> {
        let user = match users::User::builder().user(id).build() {
            Ok(user) => user,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let output: User = match user.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                println!("Gitlab is screw by amount of our requests. I'm sorry, buy you need to wait, mate");
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                return self.get_user_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, "asd"));
            }
            Ok(res) => res,
        };

        Ok(output)
    }
    fn get_group_data_by_id(&self, id: u64) -> Result<Group, Error> {
        let group = match groups::Group::builder().group(id).build() {
            Ok(group) => group,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let output: Group = match group.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                println!("Gitlab is screw by amount of our requests. I'm sorry, buy you need to wait, mate");
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                return self.get_group_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, "asd"));
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
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
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
    ) -> Result<(), Error> {
        let q = match groups::members::AddGroupMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok(()),
            Err(err) => {
                if let ApiError::Gitlab { msg } = err {
                    if msg == "Member already exists" {
                        println!("Already added");
                        return Ok(());
                    }
                    return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                } else {
                    return Err(Error::new(ErrorKind::AddrNotAvailable, err));
                };
            }
        };
    }

    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<(), Error> {
        let q = match projects::members::RemoveProjectMember::builder()
            .user(uid)
            .project(pid)
            .build()
        {
            Ok(q) => q,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok(()),
            Err(err) => {
                match err {
                    ApiError::Gitlab { msg } => {
                        if msg == "404 Not found" {
                            println!("Not a member");
                            return Ok(());
                        }
                        return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
            }
        };
    }

    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<(), Error> {
        let q = match groups::members::RemoveGroupMember::builder()
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => return Ok(()),
            Err(err) => {
                match err {
                    ApiError::Gitlab { msg } => {
                        if msg == "404 Not found" {
                            println!("Not a member");
                            return Ok(());
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
    ) -> Result<(), Error> {
        let q = match projects::members::EditProjectMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .project(pid)
            .build()
        {
            Ok(q) => q,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => print!("Fuck yeah, done"),
            Err(_) => return Err(Error::new(ErrorKind::AddrNotAvailable, "asd")),
        };
        Ok(())
    }

    fn edit_user_in_group(
        &self,
        uid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<(), Error> {
        let q = match groups::members::EditGroupMember::builder()
            .access_level(access_level.to_gitlab_access_level())
            .user(uid)
            .group(gid)
            .build()
        {
            Ok(q) => q,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };
        let _: () = match api::ignore(q).query(&self.gitlab_client) {
            Ok(_) => print!("Fuck yeah, done"),
            Err(_) => return Err(Error::new(ErrorKind::AddrNotAvailable, "asd")),
        };
        Ok(())
    }
}
