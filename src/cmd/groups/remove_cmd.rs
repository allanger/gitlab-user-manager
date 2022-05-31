use crate::args::{ArgFileName, ArgGroupId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveCmd {
    gitlab_group_id: u64,
    file_name: String,
}

impl Cmd for RemoveCmd {
    type CmdType = RemoveCmd;

    fn add() -> Command<'static> {
        Command::new("remove")
            .alias("r")
            .about("Remove group from config file")
            .arg(ArgGroupId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_group_id: ArgGroupId::parse(sub_matches)?,
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
        let mut svc = v1::GroupsService::new(self.file_name.clone());
        svc.remove(self.gitlab_group_id)?.write_state()
    }
}

