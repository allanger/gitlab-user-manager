use gitlab::{
    api::{self, groups, projects, ApiError, Query},
    Gitlab,
};
use std::io::{Error, ErrorKind, Result};

use crate::{gitlab::types::members::CustomMember, types::v1::access_level::AccessLevel};
pub(crate) trait GitlabMembersApi {
    fn add_user_to_project(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String>;
    fn add_user_to_group(&self, uid: u64, gid: u64, access_level: AccessLevel) -> Result<String>;
    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<String>;
    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<String>;
    fn add_group_to_project(&self, gid: u64, pid: u64, access_level: AccessLevel)
        -> Result<String>;
    fn remove_group_from_project(&self, gid: u64, pid: u64) -> Result<String>;
    fn edit_user_in_project(&self, uid: u64, pid: u64, access_level: AccessLevel)
        -> Result<String>;
    fn add_group_to_namespace(
        &self,
        gid: u64,
        nid: u64,
        access_level: AccessLevel,
    ) -> Result<String>;
    fn remove_group_from_namespace(&self, gid: u64, nid: u64) -> Result<String>;
    fn edit_user_in_group(&self, uid: u64, gid: u64, access_level: AccessLevel) -> Result<String>;
}

pub(crate) struct MemberGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl MemberGitlab {
    pub(crate) fn new(gitlab_client: Gitlab) -> Self {
        Self { gitlab_client }
    }
}

impl GitlabMembersApi for MemberGitlab {
    fn add_user_to_project(&self, uid: u64, pid: u64, access_level: AccessLevel) -> Result<String> {
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

    fn add_user_to_group(&self, uid: u64, gid: u64, access_level: AccessLevel) -> Result<String> {
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

    fn remove_user_from_project(&self, uid: u64, pid: u64) -> Result<String> {
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

    fn remove_user_from_group(&self, uid: u64, gid: u64) -> Result<String> {
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

    fn add_group_to_project(
        &self,
        gid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String> {
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
    fn remove_group_from_project(&self, gid: u64, pid: u64) -> Result<String> {
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

    fn edit_user_in_project(
        &self,
        uid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String> {
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

    fn add_group_to_namespace(
        &self,
        gid: u64,
        nid: u64,
        access_level: AccessLevel,
    ) -> Result<String> {
        let q = match groups::shared::AddGroupShare::builder()
            .group_access(access_level.to_gitlab_access_level())
            .id(nid)
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
                    if msg == "Shared group The group has already been shared with this group" {
                        return Ok("Already exists".to_string());
                    }
                    return Err(Error::new(ErrorKind::AddrNotAvailable, msg));
                } else {
                    return Err(Error::new(ErrorKind::AddrNotAvailable, err));
                };
            }
        };
    }

    fn remove_group_from_namespace(&self, gid: u64, nid: u64) -> Result<String> {
        let q = match groups::shared::RemoveGroupShare::builder()
            .id(nid)
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
    fn edit_user_in_group(&self, uid: u64, gid: u64, access_level: AccessLevel) -> Result<String> {
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
