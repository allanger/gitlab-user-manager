use std::io::{Error, ErrorKind};

use clap::{arg, ArgMatches, Command};

use crate::args::team_name::ArgTeamName;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::output::OutSpinner;
use crate::{cmd::args::arg_team_name, files};

pub(crate) struct AddTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
}
pub(crate) fn add_add_team_cmd() -> Command<'static> {
    return Command::new("add-team")
        .alias("at")
        .about("Add user to the team")
        .arg(ArgTeamName::add())
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(pid) => pid,
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };
    let team_name = sub_matches.value_of("team-name").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "team name is not specified",
    ));
    if team_name.is_err() {
        return Err(team_name.err().unwrap());
    }

    Ok(AddTeamCmd {
        team_name: team_name.unwrap().to_string(),
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for AddTeamCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        for user in config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {}",
                    user.name, self.team_name
                ));

                if user.teams.iter().any(|t| t.to_string() == self.team_name) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} is already a member of the team '{}'",
                            user.name, self.team_name
                        ),
                    ));
                }
                user.teams.extend([self.team_name.to_string()]);
                spinner.spinner_success("Added".to_string());

                break;
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
