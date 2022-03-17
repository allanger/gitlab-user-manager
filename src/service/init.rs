use crate::{
    gitlab::{group::GitlabGroupsApi, types::groups::Group, GitlabApiInterface},
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
            let groups_api = self.gitlab_api.groups();
            let mut all_groups: Vec<Group> = Vec::new();
            for g in groups.iter() {
                let head_group = groups_api.get_group_data_by_id(*g)?;
                let sub_groups = groups_api.get_subgroups(head_group.name.clone(), head_group.id);
                all_groups.push(head_group);
                all_groups.extend(sub_groups);
            }
            println!("{:?}", all_groups);
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
