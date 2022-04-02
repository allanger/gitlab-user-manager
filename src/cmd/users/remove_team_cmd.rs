use std::io::Error;

use clap::{ArgMatches, Command};

use crate::args::file_name::ArgFileName;
use crate::args::team_name::ArgTeamName;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::CmdOld;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;

pub(crate) struct RemoveTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
    file_name: String,
}

pub(crate) fn add_remove_team_cmd() -> Command<'static> {
    return Command::new("remove-team")
        .alias("rt")
        .about("Remove a user from the team")
        .arg(ArgUserId::add())
        .arg(ArgTeamName::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'_>, Error> {
    let gitlab_user_id = ArgUserId::parse(sub_matches)?;

    let team_name = ArgTeamName::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(RemoveTeamCmd {
        team_name,
        gitlab_user_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveTeamCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        for u in config_file.config.users.iter_mut() {
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
        config_file.write(self.file_name.clone())
    }
}
