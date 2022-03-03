use crate::args::file_name::ArgFileName;
use crate::args::project_id::ArgProjectId;
use crate::args::team_name::ArgTeamName;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::output::OutMessage;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::project::Project;
use clap::{ArgMatches, Command};

use std::io::{Error, ErrorKind};

pub(crate) fn add_remove_project_cmd() -> Command<'static> {
    return Command::new("remove-project")
        .alias("rp")
        .about("Remove a Gitlab project from the team")
        .arg(ArgTeamName::add())
        .arg(ArgProjectId::add())
        .arg(ArgFileName::add());
}
struct RemoveProjectCmd {
    file_name: String,
    team_name: String,
    gitlab_project_id: u64,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_project_id: u64 = match ArgProjectId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveProjectCmd {
        team_name,
        gitlab_project_id,
        file_name,
    })
}

impl<'a> Cmd<'a> for RemoveProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        // TODO: This should be refactored
        for team in config_file.config.teams.iter_mut() {
            if team.name == self.team_name {
                for (i, p) in team.projects.iter().enumerate() {
                    if self.gitlab_project_id == p.id {
                        let project = Project {
                            name: p.name.to_string(),
                            id: p.id,
                            ..Default::default()
                        };
                        team.projects.remove(i);
                        match config_file.write(self.file_name.clone()) {
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
