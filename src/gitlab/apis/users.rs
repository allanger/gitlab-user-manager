use gitlab::Gitlab;
use std::io::Result;

use crate::gitlab::types::users::User;
pub(crate) trait GitlabUsersApi {
    fn get_data_by_id(&self, id: u64) -> Result<User>;
}

pub(crate) struct UserGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl UserGitlab {
    pub(crate) fn new(gitlab_client: Gitlab) -> Self {
        Self { gitlab_client }
    }
}

impl GitlabUsersApi for UserGitlab {
    fn get_data_by_id(&self, id: u64) -> Result<User> {
        todo!()
    }
}
