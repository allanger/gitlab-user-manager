use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::args::team_name::ArgTeamName;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::files;
use crate::output::OutSpinner;

pub(crate) struct AddTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
}
pub(crate) fn add_add_team_cmd() -> Command<'static> {
    return Command::new("add-team")
        .alias("at")
        .about("Add user to the team")
        .arg(ArgTeamName::add())
        .arg(ArgUserId::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(AddTeamCmd {
        team_name,
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
