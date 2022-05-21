use std::io::Result;
use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgUserId, Args};
use crate::cmd::{Cmd, CmdOld};
use crate::gitlab::GitlabClient;
use crate::gitlab::{GitlabActions, GitlabApi};
use crate::output::out_message::OutMessage;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use crate::types::v1::User;

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(ArgUserId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgFileName::add());
}

pub(crate) struct CreateCmd {
    gitlab_user_id: u64,
    gitlab_url: String,
    gitlab_token: String,
    file_name: String,
}

impl Cmd for CreateCmd {
    type CmdType = CreateCmd;

    fn add() -> Command<'static> {
        Command::new("create")
            .alias("c")
            .about("Add user to the config file")
            .arg(ArgUserId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(CreateCmd {
            gitlab_user_id: ArgUserId::parse(sub_matches)?,
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
        let mut svc = v1::users::UsersService::new(
            self.file_name.clone(),
            self.file_name.clone(),
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.gitlab_user_id,
            v1::users::Action::Create,
        );
        svc.exec()?.write_state()
    }
}
