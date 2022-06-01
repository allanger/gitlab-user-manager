use console::style;
use log::info;

use crate::{
    gitlab::{apis::projects::GitlabProjectsApi, GitlabApiInterface},
    output::{out_extra::OutExtra, out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{AccessLevel, ConfigFile, Project, Team},
};
use std::io::{Error, ErrorKind, Result};

pub(crate) struct TeamsService {
    config_file: ConfigFile,
    file_path: String,
}

impl TeamsService {
    pub(crate) fn new(file_path: String) -> Self {
        info!("{}", file_path.clone());
        Self {
            config_file: ConfigFile::read(file_path.clone()).unwrap(),
            file_path,
        }
    }

    pub(crate) fn create(&mut self, team_name: String) -> Result<&mut Self> {
        let new_team = Team {
            name: team_name.to_string(),
            ..Default::default()
        };

        if self
            .config_file
            .config()
            .teams
            .iter()
            .any(|i| i.name == new_team.name)
        {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "team with this name already exists",
            ));
        }
        self.config_file.config_mut().teams.extend([new_team]);
        Ok(self)
    }

    pub(crate) fn remove(&mut self, team_name: String) -> Result<&mut Self> {
        self.config_file
            .config_mut()
            .teams
            .retain(|t| t.name != team_name);
        Ok(self)
    }

    pub(crate) fn list(&mut self, large_out: bool) -> Result<()> {
        let total = &self.config_file.config().teams.len();

        for team in self.config_file.config().teams.clone() {
            let mut message = format!("{}", team.name);
            if large_out {
                message.push_str(format!("\nprojects: {:?}\n", team.projects,).as_str());
            }
            OutMessage::message_empty(message.as_str());
        }
        OutExtra::empty_line();
        OutMessage::message_info_with_alias(
            format!("You've got {} teams here", style(total).bold().underlined()).as_str(),
        );
        Ok(())
    }
    pub(crate) fn add_to_project<T: GitlabApiInterface>(
        &mut self,
        gitlab_api: T,
        team_name: String,
        pid: u64,
        access_level: AccessLevel,
    ) -> Result<&mut Self> {
        OutMessage::message_info_with_alias("I'm getting data about the project from Gitlab");
        let projects_api = gitlab_api.projects();
        let project = projects_api.get_data_by_id(pid)?;

        for team in self.config_file.config_mut().teams.iter_mut() {
            if team.name == team_name {
                let p = Project {
                    name: project.name.to_string(),
                    id: project.id,
                    access_level: access_level,
                };
                if team.projects.iter().any(|i| i.id == p.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "The team '{}' already has an access to this project: '{}'",
                            team.name, p.name
                        ),
                    ));
                }
                team.projects.extend([p]);
                return Ok(self);
            }
        }

        let error_message = format!("The team with this name can't be found: {}", team_name);
        OutMessage::message_error(error_message.as_str());
        Err(Error::new(ErrorKind::NotFound, error_message))
    }

    pub(crate) fn remove_from_project(&mut self, team_name: String, pid: u64) -> Result<&mut Self> {
        for team in self.config_file.config_mut().teams.iter_mut() {
            if team.name == team_name {
                for (i, p) in team.projects.iter().enumerate() {
                    if pid == p.id {
                        team.projects.remove(i);
                        return Ok(self);
                    }
                }
                let error_message = format!(
                    "The team {} doesn't have access to the this project",
                    team_name,
                );
                OutMessage::message_error(error_message.as_str());
                return Err(Error::new(ErrorKind::NotFound, error_message));
            }
        }
        let error_message = format!("The team with this name can't be found: {}", team_name);
        OutMessage::message_error(error_message.as_str());
        Err(Error::new(ErrorKind::NotFound, error_message))
    }

    pub(crate) fn write_state(&self) -> Result<()> {
        match self.config_file.write(self.file_path.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
