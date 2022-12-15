use crate::args::{ArgFileName, ArgTeamName, ArgUserId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct AddTeamCmd {
    gitlab_user_id: u64,
    team_name: String,
    file_name: String,
}

impl Cmd for AddTeamCmd {
    type CmdType = AddTeamCmd;

    fn add() -> Command {
        Command::new("add-team")
            .alias("at")
            .about("Add user to the team")
            .arg(ArgTeamName::add())
            .arg(ArgUserId::add())
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

impl AddTeamCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(self.file_name.clone());
        svc.add_to_team(self.gitlab_user_id, self.team_name.clone())?
            .write_state()
    }
}
