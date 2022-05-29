use crate::args::{ArgFileName, ArgTeamName, ArgUserId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
    file_name: String,
}

impl Cmd for RemoveTeamCmd {
    type CmdType = RemoveTeamCmd;

    fn add() -> Command<'static> {
        Command::new("remove-team")
            .alias("rt")
            .about("Remove a user from the team")
            .arg(ArgUserId::add())
            .arg(ArgTeamName::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_user_id: ArgUserId::parse(sub_matches)?,
            team_name: ArgTeamName::parse(sub_matches)?,
            file_name: ArgFileName::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> std::io::Result<()> {
        match ConfigFile::read(self.file_name.clone())?.get_version()? {
            Versions::V1 => self.exec_v1(),
        }
    }
}

impl RemoveTeamCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(self.file_name.clone(), self.file_name.clone());
        svc.remove_from_team(self.gitlab_user_id, self.team_name.clone())?
            .write_state()
    }
}
