use crate::{
    gitlab::{
        apis::{groups::GitlabGroupsApi, projects::GitlabProjectsApi},
        types::group::Group,
        GitlabApiInterface, Project,
    },
    output::out_message::OutMessage,
    store::{self, Store},
    types::v1::{
        self, access_level::AccessLevel, config_file::ConfigFile, namespace::Namespace,
        state::State, user,
    },
};
use std::io::{Error, ErrorKind, Result};

// SyncService should be used to sync config with GitLab
pub(crate) struct SyncService<T: GitlabApiInterface> {
    config_file: ConfigFile,
    gitlab_api: T,
}

impl<T: GitlabApiInterface> SyncService<T> {
    pub(crate) fn new(gitlab_api: T) -> Self {
        Self {
            config_file: Default::default(),
            gitlab_api,
        }
    }

    pub(crate) fn compare(&self) -> Result<()> {
        let state = self.get_state(self.config_file.state.clone())?;
        Ok(())
        /*
         * self.state = State::get()
         * self.state.compare(self.config_file.get_config().into())
         */
    }

    pub(crate) fn apply() {}

    fn get_state(&self, state_source: String) -> Result<State> {
        match store::get_store_type(state_source) {
            Ok(store) => Ok(State::new(store.get()?)),
            Err(err) => Err(err),
        }
    }
}
