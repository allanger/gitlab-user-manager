use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgGroupId, Args};
use crate::cmd::Cmd;
use crate::gitlab::GitlabApi;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct CreateCmd {
    gitlab_group_id: u64,
    gitlab_url: String,
    gitlab_token: String,
    file_name: String,
}

impl Cmd for CreateCmd {
    type CmdType = CreateCmd;

    fn add() -> Command<'static> {
        Command::new("create")
            .alias("c")
            .about("Add group to the config file")
            .arg(ArgGroupId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            gitlab_group_id: ArgGroupId::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
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
        let mut svc = v1::GroupsService::new(self.file_name.clone());
        svc.create(
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.gitlab_group_id,
        )?
        .write_state()
    }
}
