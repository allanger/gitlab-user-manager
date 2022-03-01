use std::io::{Error, ErrorKind};

use clap::{arg, ArgMatches, Command};

use crate::cmd::Cmd;
use crate::output::OutMessage;
use crate::{cmd::args::arg_team_name, files};

pub(crate) struct RemoveTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
}
pub(crate) fn add_remove_team_cmd() -> Command<'static> {
    return Command::new("remove-team")
        .alias("rt")
        .about("Remove a user from the team")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_team_name());
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl Cmd<'_>, Error> {
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

    Ok(RemoveTeamCmd {
        team_name: team_name.unwrap().to_string(),
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for RemoveTeamCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        for u in config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, p) in u.teams.iter().enumerate() {
                    if p == &self.team_name {
                        OutMessage::message_info_clean(
                            format!("removing user {} from team {}", u.name, p).as_str(),
                        );

                        u.teams.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
