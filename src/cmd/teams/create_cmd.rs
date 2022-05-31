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

pub(crate) struct CreateCmd {
    file_name: String,
    team_name: String,
}

impl Cmd for CreateCmd {
    type CmdType = CreateCmd;

    fn add() -> Command<'static> {
        Command::new("create")
            .alias("c")
            .about("Add a team to the config file")
            .arg(ArgFileName::add())
            .arg(ArgTeamName::add())
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

impl CreateCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::TeamsService::new(self.file_name.clone());
        svc.create(self.team_name.clone())?.write_state()
    }
}
