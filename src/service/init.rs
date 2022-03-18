use crate::{
    gitlab::{
        apis::groups::GitlabGroupsApi, shared_groups, types::group::Group, GitlabApiInterface,
        Project,
    },
    output::out_message::OutMessage,
    types::v1::config_file::ConfigFile,
};
use std::io::Result;

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

    pub(crate) fn parse_groups(&mut self, groups: &Vec<u64>) -> Result<&Self> {
        if !groups.is_empty() {
            // Get all subgroups
            let groups_api = self.gitlab_api.groups();
            let mut all_groups: Vec<Group> = Vec::new();
            OutMessage::message_info_with_alias("Scrapping groups");
            for g in groups.iter() {
                let head_group = groups_api.get_data_by_id(*g)?;
                let sub_groups =
                    groups_api.get_subgroups(head_group.name.clone(), head_group.id, true);
                all_groups.push(head_group);
                all_groups.extend(sub_groups);
            }
            OutMessage::message_info_with_alias(format!("Got {} groups", groups.len()).as_str());

            // Get all projects
            OutMessage::message_info_with_alias("Scrapping projects");
            let mut projects: Vec<Project> = Vec::new();
            for g in all_groups.iter() {
                projects.extend(groups_api.get_projects(g.name.clone(), g.id));
            }
            OutMessage::message_info_with_alias(
                format!("Got {} projects", projects.len()).as_str(),
            );

            // Parse shared groups and projects
            
            /* TODO: update shared with groups api implementation
            for g in groups.iter() {
                match shared_groups::SharedWithGroups::get(g.id, gitlab.get_client()) {
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
                            let mut found = false;
                            for group in config_file.config.groups.iter_mut() {
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
                                config_file.config.groups.push(group_entry);
                            }
                        }
                    }
                    Err(_) => {
                        OutMessage::message_info_clean("This group is not shared");
                    }
                };
                // Add user if doesn't exist or add group to user if exists
                let groups_users = gitlab.get_group_members(g.name.to_string(), g.id);
                for member in groups_users.iter() {
                    let mut found = false;
                    for u in config_file.config.users.iter_mut() {
                        if u.id == member.id {
                            found = true;
                            u.namespaces.push(g.to_gum_group(member.clone()).unwrap());
                            break;
                        }
                    }
                    if !found {
                        config_file.config.users.push(user::User {
                            id: member.id,
                            name: member.name.clone(),
                            teams: Default::default(),
                            projects: Default::default(),
                            namespaces: vec![g.to_gum_group(member.clone()).unwrap()],
                        });
                    }
                }
            }
            */
        };
        Ok(self)
    }

    pub(crate) fn save(&self, file_name: &String) -> Result<()> {
        // File::save()
        Ok(())
    }

    /// Set the init service's config file.
    pub(crate) fn set_config_file(&mut self, config_file: ConfigFile) {
        self.config_file = config_file;
    }
}

#[cfg(test)]
mod tests {
    use crate::gitlab::GitlabApiMock;

    use super::InitService;

    #[test]
    fn check_group_parser() {
        let gitlab_client = GitlabApiMock;
        let groups: Vec<u64> = vec![1];
        InitService::new(gitlab_client)
            .parse_groups(&groups)
            .unwrap()
            .save(&"file_name".to_string());
    }
}
