use crate::{
    args::{ArgFileName, ArgLargeOut, Args},
    cmd::Cmd,
    service::v1,
    types::{
        common::{Version, Versions},
        v1::ConfigFile,
    },
};
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct ListCmd {
    file_name: String,
    large_out: bool,
}

impl Cmd for ListCmd {
    type CmdType = ListCmd;

    fn add() -> Command {
        Command::new("list")
            .alias("l")
            .about("List groups defined in the config file")
            .arg(ArgFileName::add())
            .arg(ArgLargeOut::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            file_name: ArgFileName::parse(sub_matches)?,
            large_out: ArgLargeOut::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> std::io::Result<()> {
        match ConfigFile::read(self.file_name.clone())?.get_version()? {
            Versions::V1 => self.exec_v1(),
        }
    }
}

impl ListCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::GroupsService::new(self.file_name.clone());
        svc.list(self.large_out)
    }
}
