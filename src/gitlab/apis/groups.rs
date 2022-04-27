use crate::{
    gitlab::{
        types::group::{Group, GroupsWithShared, SharedWithGroups},
        CustomMember, Project,
    },
    output::{out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::AccessLevel,
};
use core::time;
use gitlab::{
    api::{self, groups, ApiError, Query},
    Gitlab,
};
use mockall::predicate::*;
use mockall::*;
use std::{
    io::{Error, ErrorKind, Result},
    thread,
};
#[automock]
pub(crate) trait GitlabGroupsApi {
    fn get_data_by_id(&self, id: u64) -> Result<Group>;
    fn get_subgroups(&self, group_name: String, id: u64, recursive: bool) -> Vec<Group>;
    fn get_members(&self, name: String, id: u64) -> Vec<CustomMember>;
    fn get_shared_projects(&self, group_id: u64) -> Vec<Project>;
    fn get_projects(&self, group_name: String, id: u64) -> Vec<Project>;
    fn remove_from_namespace(&self, gid: u64, nid: u64) -> Result<String>;
    fn add_to_namespace(&self, gid: u64, nid: u64, access_level: AccessLevel) -> Result<String>;
    fn git_groups_shared_with(&self, group_id: u64) -> Result<Vec<SharedWithGroups>>;
}
pub(crate) struct GroupGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl GitlabGroupsApi for GroupGitlab {
    /// Get the group data from Gitlab
    fn get_data_by_id(&self, id: u64) -> Result<Group> {
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

    /// Get groups which are subgroups to current one
    fn get_subgroups(&self, group_name: String, id: u64, recursive: bool) -> Vec<Group> {
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
        if recursive && !head.is_empty() {
            OutSpinner::spinner_success(spinner, group_name);
            for g in head.iter() {
                let sub: Vec<Group> = self.get_subgroups(g.name.clone(), g.id, true);
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

    /// Get users that have access to this group
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

    fn remove_from_namespace(&self, gid: u64, nid: u64) -> Result<String> {
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

    fn add_to_namespace(&self, gid: u64, nid: u64, access_level: AccessLevel) -> Result<String> {
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

    fn git_groups_shared_with(&self, group_id: u64) -> Result<Vec<SharedWithGroups>> {
        let group = match groups::Group::builder().group(group_id).build() {
            Ok(group) => group,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let shared: GroupsWithShared = group.query(&self.gitlab_client).unwrap_or_else(|err| {
            OutMessage::message_info_clean(format!("{}", err).as_str());

            return GroupsWithShared::default();
        });
        let r = shared.shared_with_groups();
        Ok(r)
    }
}

pub(crate) struct GroupGitlabMock;

impl GitlabGroupsApi for GroupGitlabMock {
    fn get_data_by_id(&self, id: u64) -> Result<Group> {
        let group = Group {
            id: id,
            name: "group_name".to_string(),
            web_url: "http://localhost/group".to_string(),
        };
        Ok(group)
    }

    fn get_subgroups(&self, _: String, id: u64, _: bool) -> Vec<Group> {
        let group_1 = Group {
            id: id + 1,
            name: "group_1".to_string(),
            web_url: "http://localhost/group".to_string(),
        };
        let group_2 = Group {
            id: id + 2,
            name: "group_2".to_string(),
            web_url: "http://localhost/group".to_string(),
        };
        return vec![group_1, group_2];
    }

    fn get_members(&self, name: String, id: u64) -> Vec<CustomMember> {
        todo!()
    }

    fn get_shared_projects(&self, group_id: u64) -> Vec<Project> {
        todo!()
    }

    fn get_projects(&self, group_name: String, id: u64) -> Vec<Project> {
        todo!()
    }

    fn remove_from_namespace(&self, gid: u64, nid: u64) -> Result<String> {
        todo!()
    }

    fn add_to_namespace(&self, gid: u64, nid: u64, access_level: AccessLevel) -> Result<String> {
        todo!()
    }

    fn git_groups_shared_with(&self, group_id: u64) -> Result<Vec<SharedWithGroups>> {
        todo!()
    }
}
