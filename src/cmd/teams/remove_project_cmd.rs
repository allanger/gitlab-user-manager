use crate::cmd::Cmd;
use crate::output::OutMessage;
use crate::{
    cmd::args::{arg_project_id, arg_team_name},
    files,
    types::project::Project,
};
use clap::{ArgMatches, Command};

use std::io::{Error, ErrorKind};

pub(crate) fn add_remove_project_cmd() -> Command<'static> {
    return Command::new("remove-project")
        .alias("rp")
        .about("Remove a Gitlab project from the team")
        .arg(arg_team_name())
        .arg(arg_project_id());
}
struct RemoveProjectCmd {
    team_name: String,
    gitlab_project_id: u64,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_project_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };

    let team_name = sub_matches.value_of("team-name").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "team name is not s",
    ));
    if team_name.is_err() {
        return Err(team_name.err().unwrap());
    }

    Ok(RemoveProjectCmd {
        team_name: team_name.unwrap().to_string(),
        gitlab_project_id,
    })
}

impl<'a> Cmd<'a> for RemoveProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        // TODO: This should be refactored
        for team in config.teams.iter_mut() {
            if team.name == self.team_name {
                for (i, p) in team.projects.iter().enumerate() {
                    if self.gitlab_project_id == p.id {
                        let project = Project {
                            name: p.name.to_string(),
                            id: p.id,
                            ..Default::default()
                        };
                        team.projects.remove(i);
                        let _ = match files::write_config(config) {
                            Ok(()) => {
                                OutMessage::message_info_clean(
                                    format!(
                                        "The project {} is removed from the team {}",
                                        project.name, self.team_name
                                    )
                                    .as_str(),
                                );
                                return Ok(());
                            }
                            Err(err) => return Err(err),
                        };
                    }
                }
                let error_message = format!(
                    "The team {} doesn't have access to the this project",
                    self.team_name
                );
                OutMessage::message_error(error_message.as_str());
                return Err(Error::new(ErrorKind::NotFound, error_message));
            }
        }
        let error_message = format!("The team with this name can't be found: {}", self.team_name);
        OutMessage::message_error(error_message.as_str());
        return Err(Error::new(ErrorKind::NotFound, error_message));
    }
}
