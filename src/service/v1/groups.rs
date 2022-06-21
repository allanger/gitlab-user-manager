use console::style;

use crate::{
    gitlab::{
        apis::{groups::GitlabGroupsApi, projects::GitlabProjectsApi},
        GitlabApiInterface,
    },
    output::{out_extra::OutExtra, out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{AccessLevel, ConfigFile, Group, Namespace, Project},
};
use std::io::{Error, ErrorKind, Result};

pub(crate) struct GroupsService {
    config_file: ConfigFile,
    file_path: String,
}

impl GroupsService {
    pub(crate) fn new(file_path: String) -> Self {
        Self {
            config_file: ConfigFile::read(file_path.clone()).unwrap(),
            file_path,
        }
    }

    pub(crate) fn remove(&mut self, gid: u64) -> Result<&mut Self> {
        for (i, g) in self.config_file.config().groups.iter().enumerate() {
            if g.id == gid {
                OutMessage::message_info_clean(
                    format!("removing group {} from config", g.name.to_string()).as_str(),
                );
                self.config_file.config_mut().groups.remove(i);
                break;
            }
        }
        Ok(self)
    }

    pub(crate) fn remove_from_namespace(&mut self, gid: u64, nid: u64) -> Result<&mut Self> {
        for g in self.config_file.config_mut().groups.iter_mut() {
            if g.id == gid {
                for (i, o) in g.namespaces.iter().enumerate() {
                    if o.id == nid {
                        OutMessage::message_info_clean(
                            format!(
                                "Revoking {} access for {} from group {}",
                                o.access_level, g.name, o.name
                            )
                            .as_str(),
                        );

                        g.namespaces.remove(i);
                        break;
                    }
                }
            }
        }
        Ok(self)
    }

    pub(crate) fn create<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        gid: u64,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");
        let group_api = gitlab_api.groups();
        let group = group_api.get_data_by_id(gid)?;

        let new_group = Group {
            id: gid,
            name: group.name.to_string(),
            ..Default::default()
        };

        if self.config_file.config().groups.iter().any(|i| i.id == gid) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Group {} is already in the config file", new_group.name),
            ));
        } else {
            self.config_file.config_mut().groups.extend([new_group]);
            OutMessage::message_info_clean(
                format!("Group {} is added to the config", group.name).as_str(),
            );
            Ok(self)
        }
    }

    pub(crate) fn add_to_project<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        pid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the project from Gitlab");
        let projects_api = gitlab_api.projects();
        let project = projects_api.get_data_by_id(pid)?;

        for g in self.config_file.config_mut().groups.iter_mut() {
            if g.id == gid {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as {}",
                    g.name, project.name, access_level,
                ));
                let p = Project {
                    access_level,
                    id: project.id,
                    name: project.name,
                };
                if g.projects.iter().any(|i| i.id == p.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the group {} already has an access to this project: '{}'",
                            g.name, p.name
                        ),
                    ));
                }
                g.projects.extend([p]);
                spinner.spinner_success("Added".to_string());
                break;
            }
        }
        Ok(self)
    }

    pub(crate) fn add_to_namespace<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        nid: u64,
        gid: u64,
        access_level: AccessLevel,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");
        let group_api = gitlab_api.groups();
        let namespace = group_api.get_data_by_id(nid)?;

        for g in self.config_file.config_mut().groups.iter_mut() {
            if g.id == gid {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as owner",
                    g.name, namespace.name
                ));
                let o = Namespace {
                    name: namespace.name.to_string(),
                    access_level,
                    id: namespace.id,
                    url: namespace.web_url.to_string(),
                };
                if g.namespaces.iter().any(|i| i.id == o.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the group {} is already a {} in this namespace: '{}'",
                            g.name, o.access_level, o.name
                        ),
                    ));
                }
                g.namespaces.extend([o]);
                spinner.spinner_success("Added".to_string());
            }
        }
        Ok(self)
    }

    pub(crate) fn list(&mut self, large_out: bool) -> Result<()> {
        let total = &self.config_file.config().groups.len();

        for g in self.config_file.config().groups.clone() {
            let mut message = format!("{} - {}", g.id, g.name);
            if large_out {
                message.push_str(
                    format!(
                        "\nprojects: {:?}\ngroups: {:?}\n",
                        g.projects, g.namespaces
                    )
                    .as_str(),
                );
            }
            OutMessage::message_empty(message.as_str());
        }
        OutExtra::empty_line();
        OutMessage::message_info_with_alias(
            format!(
                "You've got {} groups here",
                style(total).bold().underlined()
            )
            .as_str(),
        );
        Ok(())
    }

    pub(crate) fn remove_from_project(&mut self, gid: u64, pid: u64) -> Result<&mut Self> {
        for g in self.config_file.config_mut().groups.iter_mut() {
            if g.id == gid {
                for (i, p) in g.projects.iter().enumerate() {
                    if p.id == pid {
                        OutMessage::message_info_clean(
                            format!("removing group {} from project {}", g.name, p.name).as_str(),
                        );

                        g.projects.remove(i);
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
