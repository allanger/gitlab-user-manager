use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::args::{ArgFileName, ArgTeamName, ArgUserId, Args};
use crate::cmd::CmdOld;
use crate::output::out_spinner::OutSpinner;
use crate::types::v1::config_file::ConfigFile;

pub(crate) struct AddTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
    file_name: String,
}
pub(crate) fn add_add_team_cmd() -> Command<'static> {
    return Command::new("add-team")
        .alias("at")
        .about("Add user to the team")
        .arg(ArgTeamName::add())
        .arg(ArgUserId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_user_id = ArgUserId::parse(sub_matches)?;
    let team_name = ArgTeamName::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(AddTeamCmd {
        team_name,
        gitlab_user_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for AddTeamCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        for user in config_file.config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {}",
                    user.name, self.team_name
                ));

                if user.teams.iter().any(|t| *t == self.team_name) {
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
        config_file.write(self.file_name.clone())
    }
}
