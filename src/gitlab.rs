pub(crate) mod group;
pub mod shared_groups;
pub(crate) mod shared_projects;
pub(crate) mod types;

use core::time;
use std::{
    io::{Error, ErrorKind},
    thread,
};

use gitlab::{
    api::{self, groups, projects, users, ApiError, Query},
    Gitlab,
};
// use gitlab::AccessLevel;
use serde::Deserialize;
use tabled::Tabled;

use crate::{
    gitlab::group::GroupGitlab,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{access_level::AccessLevel, namespace, project},
};

use self::group::{GitlabGroupsApi, GroupGitlabMock};

pub(crate) trait GitlabApiInterface {
    type Groups: GitlabGroupsApi;
    fn groups(&self) -> Self::Groups;
}

pub(crate) struct GitlabApi {
    pub(crate) gitlab_client: Gitlab,
}

impl GitlabApi {
    pub(crate) fn new(gitlab_url: &String, gitlab_token: &String) -> Result<Self, Error> {
        match Gitlab::new(gitlab_url.clone(), gitlab_token.clone()) {
            Ok(gitlab_client) => Ok(GitlabApi { gitlab_client }),
            Err(err) => return Err(Error::new(ErrorKind::Other, err)),
        }
    }
}

impl GitlabApiInterface for GitlabApi {
    type Groups = GroupGitlab;
    fn groups(&self) -> Self::Groups {
        return GroupGitlab {
            gitlab_client: self.gitlab_client.clone(),
        };
    }
}
pub(crate) struct GitlabApiMock;

impl GitlabApiInterface for GitlabApiMock {
    type Groups = GroupGitlabMock;
    fn groups(&self) -> Self::Groups {
        GroupGitlabMock {}
    }
}

pub(crate) struct GitlabClient {
    gitlab_client: Gitlab,
}

pub(crate) struct GitlabClientMock;

impl GitlabClientApi for GitlabClient {
    type Client = Gitlab;

    fn get_client(&self) -> Self::Client {
        self.gitlab_client.clone()
    }
}

pub(crate) trait GitlabClientApi {
    type Client;
    fn get_client(&self) -> Self::Client;
}

/*
======================================================================================================================
======================================================== LEGACY ======================================================
======================================================================================================================
*/
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
    fn add_group_to_namespace(
        &self,
        gid: u64,
        nid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
    fn add_group_to_project(
        &self,
        gid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error>;
    fn remove_group_from_namespace(&self, gid: u64, nid: u64) -> Result<String, Error>;
    fn remove_group_from_project(&self, gid: u64, pid: u64) -> Result<String, Error>;
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

    fn get_subgroups(&self, group_name: String, id: u64) -> Vec<Group>;
    fn get_projects(&self, group_name: String, id: u64) -> Vec<Project>;
    fn get_project_members(&self, name: String, id: u64) -> Vec<CustomMember>;
    fn get_group_members(&self, name: String, id: u64) -> Vec<CustomMember>;
    fn get_shared_projects(&self, group_id: u64) -> Vec<Project>;
}

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl Project {
    pub(crate) fn to_gum_project(&self, member: CustomMember) -> Result<project::Project, Error> {
        let project = project::Project {
            id: self.id,
            name: self.name.clone(),
            access_level: AccessLevel::from_gitlab_access_level(member.access_level),
        };
        Ok(project)
    }
}

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) web_url: String,
}
#[derive(Debug, Deserialize, Tabled, Clone)]
pub(crate) struct CustomMember {
    pub(crate) id: u64,
    pub(crate) access_level: gitlab::AccessLevel,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

#[derive(Debug, Deserialize, Tabled, Clone)]

pub(crate) struct Group {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl Group {
    pub(crate) fn to_gum_group(&self, member: CustomMember) -> Result<namespace::Namespace, Error> {
        let group = namespace::Namespace {
            id: self.id,
            name: self.name.clone(),
            url: self.web_url.clone(),
            access_level: AccessLevel::from_gitlab_access_level(member.access_level),
        };
        Ok(group)
    }
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

    fn get_subgroups(&self, group_name: String, id: u64) -> Vec<Group> {
        let spinner = OutSpinner::spinner_start("Getting subgroups".to_string());

        let mut groups: Vec<Group> = Vec::new();
        let query = match groups::subgroups::GroupSubgroups::builder()
            .group(id)
            .all_available(true)
            .build()
        {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let head: Vec<Group> = query.query(&self.gitlab_client).unwrap();
        if !head.is_empty() {
            OutSpinner::spinner_success(spinner, group_name);
            for g in head.iter() {
                let sub: Vec<Group> = self.get_subgroups(g.name.clone(), g.id);
                if !sub.is_empty() {
                    groups.extend(sub);
                }
            }
        } else {
            OutSpinner::spinner_success(spinner, group_name);
        }
        groups.extend(head);
        groups
    }

    fn get_projects(&self, group_name: String, id: u64) -> Vec<Project> {
        let spinner = OutSpinner::spinner_start(format!("Getting projects from {}", group_name));
        let query = match groups::projects::GroupProjects::builder()
            .group(id)
            .with_shared(false)
            .build()
        {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let projects: Vec<Project> = query.query(&self.gitlab_client).unwrap();
        OutSpinner::spinner_success(spinner, format!("Got {}", projects.len()));

        projects
    }

    fn get_group_members(&self, name: String, id: u64) -> Vec<CustomMember> {
        let spinner = OutSpinner::spinner_start(format!("Getting users from {}", name));
        let query = match groups::members::GroupMembers::builder().group(id).build() {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let users: Vec<CustomMember> = query.query(&self.gitlab_client).unwrap();
        OutSpinner::spinner_success(spinner, "Done".to_string());
        users
    }

    fn get_project_members(&self, name: String, id: u64) -> Vec<CustomMember> {
        let spinner = OutSpinner::spinner_start(format!("Getting projects from {}", name));
        let query = match projects::members::ProjectMembers::builder()
            .project(id)
            .build()
        {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let users: Vec<CustomMember> = query.query(&self.gitlab_client).unwrap();
        OutSpinner::spinner_success(spinner, "Done".to_string());
        users
    }

    fn get_shared_projects(&self, group_id: u64) -> Vec<Project> {
        let query = match groups::shared::GroupSharedProjects::builder()
            .id(group_id)
            .build()
        {
            Ok(q) => q,
            Err(_) => todo!(),
        };
        let users: Vec<Project> = query.query(&self.gitlab_client).unwrap();
        print!("{:?}", users);
        users
    }

    fn add_group_to_namespace(
        &self,
        gid: u64,
        nid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
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

    fn add_group_to_project(
        &self,
        gid: u64,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<String, Error> {
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

    fn remove_group_from_namespace(&self, gid: u64, nid: u64) -> Result<String, Error> {
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

    fn remove_group_from_project(&self, gid: u64, pid: u64) -> Result<String, Error> {
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
