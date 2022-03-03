use std::io::Error;

use clap::{ArgMatches, Command};

use crate::args::team_name::ArgTeamName;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::files;
use crate::output::OutMessage;

pub(crate) struct RemoveTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
}
pub(crate) fn add_remove_team_cmd() -> Command<'static> {
    return Command::new("remove-team")
        .alias("rt")
        .about("Remove a user from the team")
        .arg(ArgUserId::add())
        .arg(ArgTeamName::add());
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl Cmd<'_>, Error> {
    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveTeamCmd {
        team_name,
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
