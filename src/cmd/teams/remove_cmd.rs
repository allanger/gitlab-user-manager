use crate::{
    args::{ArgFileName, ArgTeamName, Args},
    cmd::Cmd,
    service::v1,
    types::{
        common::{Version, Versions},
        v1::ConfigFile,
    },
};
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveCmd {
    team_name: String,
    file_name: String,
}

impl Cmd for RemoveCmd {
    type CmdType = RemoveCmd;

    fn add() -> Command {
        Command::new("remove")
            .alias("r")
            .about("Remove the team from the config file")
            .arg(ArgTeamName::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
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

impl RemoveCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::TeamsService::new(self.file_name.clone());
        svc.remove(self.team_name.clone())?.write_state()
    }
}
