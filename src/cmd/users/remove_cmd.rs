use crate::args::{ArgFileName, ArgUserId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveCmd {
    gitlab_user_id: u64,
    file_name: String,
}

impl Cmd for RemoveCmd {
    type CmdType = RemoveCmd;

    fn add() -> Command {
        Command::new("remove")
            .alias("r")
            .about("Remove user from config file")
            .arg(ArgUserId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_user_id: ArgUserId::parse(sub_matches)?,
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
        let mut svc = v1::users::UsersService::new(self.file_name.clone());
        svc.remove(self.gitlab_user_id)?.write_state()
    }
}
