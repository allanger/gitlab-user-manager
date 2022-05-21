use std::io::Result;

use clap::{ArgMatches, Command};

use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgUserId, Args};
use crate::cmd::Cmd;
use crate::gitlab::GitlabApi;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;

pub(crate) struct RemoveCmd {
    gitlab_user_id: u64,
    gitlab_url: String,
    gitlab_token: String,
    file_name: String,
}

impl Cmd for RemoveCmd {
    type CmdType = RemoveCmd;

    fn add() -> Command<'static> {
        Command::new("remove")
            .alias("r")
            .about("Remove user from config file")
            .arg(ArgUserId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(RemoveCmd {
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

impl RemoveCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(
            self.file_name.clone(),
            self.file_name.clone(),
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.gitlab_user_id,
            v1::users::Action::Remove,
        );
        svc.exec()?.write_state()
    }
}
