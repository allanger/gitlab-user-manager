use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgProjectId, ArgUserId, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveProjectCmd {
    gitlab_user_id: u64,
    gitlab_project_id: u64,
    file_name: String,
}

impl Cmd for RemoveProjectCmd {
    type CmdType = RemoveProjectCmd;

    fn add() -> Command<'static> {
        Command::new("remove-project")
            .alias("rp")
            .about("Remove user from the project")
            .arg(ArgUserId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgProjectId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_project_id: ArgProjectId::parse(sub_matches)?,
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

impl RemoveProjectCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(self.file_name.clone(), self.file_name.clone());
        svc.remove_from_project(self.gitlab_user_id, self.gitlab_project_id)?
            .write_state()
    }
}
