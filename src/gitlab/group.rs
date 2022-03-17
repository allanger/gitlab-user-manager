use crate::output::{out_message::OutMessage, out_spinner::OutSpinner};
use core::time;
use gitlab::{
    api::{groups, ApiError, Query},
    Gitlab,
};
use std::{
    io::{Error, ErrorKind, Result},
    thread,
};

use super::types::groups::Group;

pub(crate) trait GitlabGroupsApi {
    fn get_group_data_by_id(&self, id: u64) -> Result<Group>;
    fn get_subgroups(&self, group_name: String, id: u64) -> Vec<Group>;
}

pub(crate) struct GroupGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl GitlabGroupsApi for GroupGitlab {
    fn get_group_data_by_id(&self, id: u64) -> Result<Group> {
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
            for g in head.iter() {
                let sub: Vec<Group> = self.get_subgroups(g.name.clone(), g.id);
                if !sub.is_empty() {
                    groups.extend(sub);
                }
            }
        }
        OutSpinner::spinner_success(spinner, group_name);
        groups.extend(head);
        groups
    }
}

pub(crate) struct GroupGitlabMock;

impl GitlabGroupsApi for GroupGitlabMock {
    fn get_group_data_by_id(&self, id: u64) -> Result<Group> {
        let group = Group {
            id: id,
            name: "group_name".to_string(),
            web_url: "http://localhost/group".to_string(),
        };
        Ok(group)
    }

    fn get_subgroups(&self, group_name: String, id: u64) -> Vec<Group> {
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
}
