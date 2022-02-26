use std::io::{Error, ErrorKind};

use clap::{arg, Command, ArgMatches};

use crate::cmd::Cmd;
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
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
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
            Err(_error) => return Err(_error),
        };
        for u in config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, p) in u.teams.iter().enumerate() {
                    if p == &self.team_name {
                        println!("removing user {} from team {}", u.name, p);
                        u.teams.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
