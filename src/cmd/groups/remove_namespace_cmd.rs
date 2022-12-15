use crate::args::{ArgFileName, ArgGroupId, ArgNamespaceId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveNamespaceCmd {
    gitlab_group_id: u64,
    gitlab_namespace_id: u64,
    file_name: String,
}

impl Cmd for RemoveNamespaceCmd {
    type CmdType = RemoveNamespaceCmd;

    fn add() -> Command {
        Command::new("remove-namespace")
            .alias("rn")
            .about("Remove group from the namespace")
            .arg(ArgGroupId::add())
            .arg(ArgNamespaceId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_namespace_id: ArgNamespaceId::parse(sub_matches)?,
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

impl RemoveNamespaceCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::GroupsService::new(self.file_name.clone());
        svc.remove_from_namespace(self.gitlab_group_id, self.gitlab_namespace_id)?
            .write_state()
    }
}
