use console::style;

use crate::{
    gitlab::{
        apis::{groups::GitlabGroupsApi, projects::GitlabProjectsApi, users::GitlabUsersApi},
        GitlabApiInterface,
    },
    output::{out_extra::OutExtra, out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{AccessLevel, ConfigFile, Namespace, Project, User},
};
use std::io::{Error, ErrorKind, Result};

pub(crate) struct UsersService {
    config_file: ConfigFile,
    file_path: String,
}

impl UsersService {
    pub(crate) fn new(file_path: String) -> Self {
        Self {
            config_file: ConfigFile::read(file_path.clone()).unwrap(),
            file_path,
        }
    }

    pub(crate) fn list(&mut self, large_out: bool) -> Result<()> {
        let total = &self.config_file.config().users.len();

        for u in self.config_file.config().users.clone() {
            let mut message = format!("{} - {}", u.id, u.name);
            if large_out {
                message.push_str(
                    format!(
                        "\nprojects: {:?}\nteams: {:?}\ngroups: {:?}\n",
                        u.projects, u.teams, u.namespaces
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

    pub(crate) fn create<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        user_id: u64,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the user from Gitlab");
        let users_api = gitlab_api.users();
        let user = users_api.get_data_by_id(user_id)?;

        let new_user = User {
            id: user_id,
            name: user.name.to_string(),
            ..Default::default()
        };

        if self
            .config_file
            .config()
            .users
            .iter()
            .any(|i| i.id == user_id)
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

    pub(crate) fn remove(&mut self, user_id: u64) -> Result<&mut Self> {
        for (i, u) in self.config_file.config().users.iter().enumerate() {
            if u.id == user_id {
                let u = User {
                    id: u.id,
                    name: u.name.to_string(),
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

    pub(crate) fn add_to_namespace<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        gid: u64,
        uid: u64,
        access_level: AccessLevel,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");
        let group_api = gitlab_api.groups();
        let namespace = group_api.get_data_by_id(gid)?;

        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as owner",
                    u.name, namespace.name
                ));
                let o = Namespace {
                    name: namespace.name.to_string(),
                    access_level,
                    id: namespace.id,
                    url: namespace.web_url.to_string(),
                };
                if u.namespaces.iter().any(|i| i.id == o.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} is already owner of this group: '{}'",
                            u.name, o.name
                        ),
                    ));
                }
                u.namespaces.extend([o]);
                spinner.spinner_success("Added".to_string());
            }
        }
        Ok(self)
    }

    pub(crate) fn add_to_team(&mut self, uid: u64, team_name: String) -> Result<&mut Self> {
        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                let spinner =
                    OutSpinner::spinner_start(format!("Adding {} to {}", u.name, team_name));

                if u.teams.iter().any(|t| *t == team_name) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} is already a member of the team '{}'",
                            u.name, team_name
                        ),
                    ));
                }
                u.teams.extend([team_name.to_string()]);
                spinner.spinner_success("Added".to_string());

                break;
            }
        }
        Ok(self)
    }

    pub(crate) fn add_to_project<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        pid: u64,
        uid: u64,
        access_level: AccessLevel,
    ) -> Result<&mut Self> {
        let projects_api = gitlab_api.projects();
        OutMessage::message_info_with_alias("I'm getting data about the project from Gitlab");
        let project = projects_api.get_data_by_id(pid)?;

        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as {}",
                    u.name, project.name, access_level,
                ));

                let p = Project {
                    access_level,
                    id: project.id,
                    name: project.name,
                };
                if u.projects.iter().any(|i| i.id == p.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} already has an access to this project: '{}'",
                            u.name, p.name
                        ),
                    ));
                }

                u.projects.extend([p]);
                spinner.spinner_success("Added".to_string());
                break;
            }
        }

        Ok(self)
    }

    pub(crate) fn remove_from_namespace(&mut self, uid: u64, gid: u64) -> Result<&mut Self> {
        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                for (i, o) in u.namespaces.iter().enumerate() {
                    if o.id == gid {
                        OutMessage::message_info_clean(
                            format!("Removing ownership on {} for user {}", o.name, u.name)
                                .as_str(),
                        );

                        u.namespaces.remove(i);
                        break;
                    }
                }
            }
        }
        Ok(self)
    }

    pub(crate) fn remove_from_project(&mut self, uid: u64, pid: u64) -> Result<&mut Self> {
        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                for (i, p) in u.projects.iter().enumerate() {
                    if p.id == pid {
                        OutMessage::message_info_clean(
                            format!("removing user {} from project {}", u.name, p.name).as_str(),
                        );

                        u.projects.remove(i);
                        break;
                    }
                }
            }
        }
        Ok(self)
    }

    pub(crate) fn remove_from_team(&mut self, uid: u64, team_name: String) -> Result<&mut Self> {
        for u in self.config_file.config_mut().users.iter_mut() {
            if u.id == uid {
                for (i, p) in u.teams.iter().enumerate() {
                    if p == &team_name {
                        OutMessage::message_info_clean(
                            format!("removing user {} from team {}", u.name, p).as_str(),
                        );

                        u.teams.remove(i);
                        break;
                    }
                }
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
