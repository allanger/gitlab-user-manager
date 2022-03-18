use crate::{
    gitlab::{CustomMember, Project},
    output::{out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::access_level::AccessLevel,
};
use core::time;
use gitlab::{
    api::{self, groups, projects, ApiError, Query},
    Gitlab,
};
use std::{
    io::{Error, ErrorKind, Result},
    thread,
};

pub(crate) trait GitlabProjectsApi {
    fn get_data_by_id(&self, id: u64) -> Result<Project>;
    fn add_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String>;
    fn remove_user(&self, uid: u64, pid: u64) -> Result<String>;
    fn edit_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String>;
    fn get_members(&self, name: String, id: u64) -> Vec<CustomMember>;
    fn share_with_group(&self, gid: u64, pid: u64, access_level: AccessLevel) -> Result<String>;
    fn stop_sharing_with_group(&self, gid: u64, pid: u64) -> Result<String>;
}

pub(crate) struct ProjectsGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl GitlabProjectsApi for ProjectsGitlab {
    fn get_data_by_id(&self, id: u64) -> Result<Project> {
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
                                return self.get_data_by_id(id);
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

    fn add_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
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

    fn remove_user(&self, uid: u64, pid: u64) -> Result<String> {
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

    fn edit_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
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

    fn get_members(&self, name: String, id: u64) -> Vec<CustomMember> {
        let spinner = OutSpinner::spinner_start(format!("Getting users from {}", name));
        let query = match groups::members::GroupMembers::builder().group(id).build() {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let users: Vec<CustomMember> = query.query(&self.gitlab_client).unwrap();
        OutSpinner::spinner_success(spinner, "Done".to_string());
        users
    }

    fn share_with_group(&self, gid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
        let q = match projects::share::AddProjectShare::builder()
            .group_access(access_level.to_gitlab_access_level())
            .project(pid)
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
                    if msg == "Group already shared with this group" {
                        return Ok("Already exists".to_string());
                    }
                    return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                } else {
                    return Err(Error::new(ErrorKind::AddrNotAvailable, err));
                };
            }
        };
    }

    fn stop_sharing_with_group(&self, gid: u64, pid: u64) -> Result<String> {
        let q = match projects::share::RemoveProjectShare::builder()
            .project(pid)
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
                        if msg == "404 Group Link Not Found" {
                            return Ok("Not found".to_string());
                        }
                        return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
            }
        };
    }
}

pub(crate) struct ProjectsGitlabMock;

impl GitlabProjectsApi for ProjectsGitlabMock {
    fn get_data_by_id(&self, id: u64) -> Result<Project> {
        todo!()
    }

    fn add_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
        todo!()
    }

    fn remove_user(&self, uid: u64, pid: u64) -> Result<String> {
        todo!()
    }

    fn edit_user(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
        todo!()
    }

    fn get_members(&self, name: String, id: u64) -> Vec<CustomMember> {
        todo!()
    }

    fn share_with_group(&self, gid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
        todo!()
    }

    fn stop_sharing_with_group(&self, gid: u64, pid: u64) -> Result<String> {
        todo!()
    }
}
