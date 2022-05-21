use crate::{
    gitlab::{
        apis::{groups::GitlabGroupsApi, projects::GitlabProjectsApi, users::GitlabUsersApi},
        types::group::Group,
        GitlabApiInterface, Project,
    },
    output::out_message::OutMessage,
    types::v1::{self, AccessLevel, ConfigFile, Namespace, User},
};
use std::io::{Error, ErrorKind, Result};

//InitService should be used to generate an initial config file
pub(crate) struct UsersService<T: GitlabApiInterface> {
    config_file: ConfigFile,
    file_path: String,
    gitlab_api: T,
    user_id: u64,
    action: Action,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    Create,
    Remove,
}

impl<T: GitlabApiInterface> UsersService<T> {
    pub(crate) fn new(
        config_path: String,
        file_path: String,
        gitlab_api: T,
        user_id: u64,
        action: Action,
    ) -> Self {
        Self {
            config_file: ConfigFile::read(config_path).unwrap(),
            file_path,
            gitlab_api,
            user_id,
            action,
        }
    }

    pub(crate) fn exec(&mut self) -> Result<&mut Self> {
        match self.action {
            Action::Create => {
                OutMessage::message_info_with_alias("I'm getting data about the user from Gitlab");
                // let user = gitlab.get_user_data_by_id(self.gitlab_user_id)?;
                let users_api = self.gitlab_api.users();

                let user = users_api.get_data_by_id(self.user_id)?;

                let new_user = User {
                    id: self.user_id,
                    name: user.name.to_string(),
                    ..Default::default()
                };

                if self
                    .config_file
                    .config()
                    .users
                    .iter()
                    .any(|i| i.id == self.user_id)
                {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!("User {} is already in the config file", new_user.name),
                    ));
                } else {
                    self.config_file.config_mut().users.extend([new_user]);
                    OutMessage::message_info_clean(
                        format!("User {} is added to the config", user.name).as_str(),
                    );
                    Ok(self)
                }
            }
            Action::Remove => {
                for (i, user) in self.config_file.config().users.iter().enumerate() {
                    if user.id == self.user_id {
                        let u = User {
                            id: user.id,
                            name: user.name.to_string(),
                            ..Default::default()
                        };
                        OutMessage::message_info_clean(
                            format!("removing user {} from config", u.name).as_str(),
                        );
                        self.config_file.config_mut().users.remove(i);
                        break;
                    }
                }
                Ok(self)
            }
        }
    }
    pub(crate) fn write_state(&self) -> Result<()> {
        match self.config_file.write(self.file_path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
