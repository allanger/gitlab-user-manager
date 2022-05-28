use crate::{
    gitlab::{apis::users::GitlabUsersApi, GitlabApiInterface},
    output::{out_extra::OutExtra, out_message::OutMessage},
    types::v1::{ConfigFile, User},
};
use std::io::{Error, ErrorKind, Result};

//InitService should be used to generate an initial config file
// pub(crate) struct UsersService<T: GitlabApiInterface> {
pub(crate) struct UsersService {
    config_file: ConfigFile,
    file_path: String,
    user_id: u64,
}

impl UsersService {
    pub(crate) fn new(config_path: String, file_path: String, user_id: u64) -> Self {
        Self {
            config_file: ConfigFile::read(config_path).unwrap(),
            file_path,
            user_id,
        }
    }

    pub(crate) fn list(&mut self) -> Result<()> {
        let total = &self.config_file.config().users.len();

        for user in self.config_file.config().users.clone() {
            let mut message = format!("{} - {}", user.id, user.name);
            if self.large_out {
                message.push_str(
                    format!(
                        "\nprojects: {:?}\nteams: {:?}\ngroups: {:?}\n",
                        user.projects, user.teams, user.namespaces
                    )
                    .as_str(),
                );
            }
            OutMessage::message_empty(message.as_str());
        }
        OutExtra::empty_line();
        OutMessage::message_info_with_alias(
            format!("You've got {} users here", style(total).bold().underlined()).as_str(),
        );
        Ok(())
    }

    pub(crate) fn create<T: GitlabApiInterface>(&mut self, gitlab_api: T) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the user from Gitlab");
        let users_api = gitlab_api.users();

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

    pub(crate) fn remove(&mut self) -> Result<&mut Self> {
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

    pub(crate) fn write_state(&self) -> Result<()> {
        match self.config_file.write(self.file_path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
