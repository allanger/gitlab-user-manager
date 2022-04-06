use crate::{
    gitlab::{
        apis::{groups::GitlabGroupsApi, projects::GitlabProjectsApi},
        types::group::Group,
        GitlabApiInterface, Project,
    },
    output::out_message::OutMessage,
    types::v1::{
        self, access_level::AccessLevel, config_file::ConfigFile, namespace::Namespace, user,
    },
};
use std::io::{Error, ErrorKind, Result};

//InitService should be used to generate an initial config file
pub(crate) struct InitService<T: GitlabApiInterface> {
    config_file: ConfigFile,
    gitlab_api: T,
}

impl<T: GitlabApiInterface> InitService<T> {
    pub(crate) fn new(gitlab_api: T) -> Self {
        Self {
            config_file: Default::default(),
            gitlab_api,
        }
    }

    fn get_groups(&self, groups: &Vec<u64>) -> Result<Vec<Group>> {
        let groups_api = self.gitlab_api.groups();
        let mut all_groups: Vec<Group> = Vec::new();
        OutMessage::message_info_with_alias("Scrapping groups");
        for g in groups.iter() {
            let head_group = groups_api.get_data_by_id(*g)?;
            let sub_groups = groups_api.get_subgroups(head_group.name.clone(), head_group.id, true);
            all_groups.push(head_group);
            all_groups.extend(sub_groups);
        }
        OutMessage::message_info_with_alias(format!("Got {} groups", all_groups.len()).as_str());
        Ok(all_groups)
    }

    fn get_projects(&self, groups: &Vec<Group>) -> Result<Vec<Project>> {
        let groups_api = self.gitlab_api.groups();
        OutMessage::message_info_with_alias("Scrapping projects");
        let mut projects: Vec<Project> = Vec::new();
        for g in groups.iter() {
            projects.extend(groups_api.get_projects(g.name.clone(), g.id));
        }
        OutMessage::message_info_with_alias(format!("Got {} projects", projects.len()).as_str());
        return Ok(projects);
    }

    fn get_groups_members(&mut self, groups: &Vec<Group>) -> Result<()> {
        for g in groups.iter() {
            let groups_api = self.gitlab_api.groups();
            match groups_api.git_groups_shared_with(g.id) {
                Ok(group) => {
                    for ns in group.iter() {
                        let item = Namespace {
                            name: g.name.clone(),
                            access_level: AccessLevel::from_gitlab_access_level(
                                ns.group_access_level,
                            ),
                            id: g.id,
                            url: g.web_url.clone(),
                        };
                        // TODO: Use a HashMap here to avoid a loop

                        let mut found = false;
                        for group in self.config_file.config.groups.iter_mut() {
                            if ns.group_id == group.id {
                                found = true;
                                group.namespaces.push(item.clone());
                            }
                        }
                        if !found {
                            let group_entry = v1::group::Group {
                                name: ns.group_name.clone(),
                                id: ns.group_id,
                                projects: Default::default(),
                                namespaces: vec![item],
                            };
                            self.config_file.config.groups.push(group_entry);
                        }
                    }
                }
                Err(_) => {
                    OutMessage::message_info_clean("This group is not shared");
                }
            };
            let groups_users = groups_api.get_members(g.name.to_string(), g.id);
            for member in groups_users.iter() {
                // TODO: Use a HashMap here to avoid a loop
                let mut found = false;
                for u in self.config_file.config.users.iter_mut() {
                    if u.id == member.id {
                        found = true;
                        u.namespaces.push(g.to_gum_group(member.clone()).unwrap());
                        break;
                    }
                }
                if !found {
                    self.config_file.config.users.push(user::User {
                        id: member.id,
                        name: member.name.clone(),
                        teams: Default::default(),
                        projects: Default::default(),
                        namespaces: vec![g.to_gum_group(member.clone()).unwrap()],
                    });
                }
            }
        }
        Ok(())
    }

    fn get_projects_members(&mut self, projects: &Vec<Project>) -> Result<()> {
        let projects_api = self.gitlab_api.projects();
        for p in projects.iter() {
            // Add user if doesn't exist or add group to user if exists
            // TODO: Use a HashMap here to avoid a loop
            match projects_api.get_groups_shared_with(p.id) {
                Ok(group) => {
                    for ns in group.iter() {
                        let item = v1::project::Project {
                            id: p.id,
                            name: p.name.clone(),
                            access_level: AccessLevel::from_gitlab_access_level(
                                ns.group_access_level,
                            ),
                        };
                        let mut found = false;
                        for group in self.config_file.config.groups.iter_mut() {
                            if ns.group_id == group.id {
                                found = true;
                                group.projects.push(item.clone());
                            }
                        }
                        if !found {
                            let group_entry = v1::group::Group {
                                name: ns.group_name.clone(),
                                id: ns.group_id,
                                namespaces: Default::default(),
                                projects: vec![item],
                            };
                            self.config_file.config.groups.push(group_entry);
                        }
                    }
                }
                Err(_) => {
                    OutMessage::message_info_clean("This project is not shared");
                }
            };
            let projects_users = projects_api.get_members(p.name.to_string(), p.id);
            for member in projects_users.iter() {
                // TODO: Use a HashMap here to avoid a loop
                let mut found = false;
                for u in self.config_file.config.users.iter_mut() {
                    if u.id == member.id {
                        found = true;
                        u.projects.push(p.to_gum_project(member.clone()).unwrap());
                        break;
                    }
                }
                if !found {
                    self.config_file.config.users.push(user::User {
                        id: member.id,
                        name: member.name.clone(),
                        projects: vec![p.to_gum_project(member.clone()).unwrap()],
                        teams: Default::default(),
                        namespaces: Default::default(),
                    });
                }
            }
        }
        Ok(())
    }
    pub(crate) fn generate_config(&mut self, groups: &Vec<u64>) -> Result<&Self> {
        if !groups.is_empty() {
            let groups = self.get_groups(groups)?;
            let projects = self.get_projects(&groups)?;
            self.get_groups_members(&groups)?;
            self.get_projects_members(&projects)?;
        };
        Ok(self)
    }

    pub(crate) fn save(&self, file_name: &String) -> Result<()> {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_name.clone())
        {
            Ok(file) => file,
            Err(err) => {
                return match err.kind() {
                    ErrorKind::AlreadyExists => {
                        return Err(Error::new(
                            err.kind(),
                            "config file already exists in specified directory",
                        ))
                    }
                    _ => Err(Error::new(ErrorKind::AlreadyExists, err)),
                }
            }
        };

        match self.config_file.write(file_name.clone()) {
            Ok(_) => {
                OutMessage::message_empty(
                    format!(
                        "Config file is generated, check it out\n $ cat {}",
                        file_name.clone()
                    )
                    .as_str(),
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::{mock, predicate};

    use crate::gitlab::{
        apis::groups::{GroupGitlabMock, MockGitlabGroupsApi},
        types::group::Group,
        GitlabApiInterface, GitlabApiMock,
    };

    use super::InitService;

    #[test]
    fn check_group_parser() {
        // let gitlab_client = GitlabApiMock;
        // let groups: Vec<u64> = vec![1];
        // InitService::new(gitlab_client)
        // .parse_groups(&groups)
        // .unwrap()
        // .save(&"file_name".to_string());
        // let mock = MockGitlabGroupsApi
        // mock.expect_get_data_by_id()
        // .with(predicate::eq(4))
        // .times(1)
        // .returning(|x| {
        // Ok(Group {
        // id: x,
        // name: "lala".to_string(),
        // web_url: "lala".to_string(),
        // })
        // });
        println!("");
        // InitService::new(mock);
    }
}
