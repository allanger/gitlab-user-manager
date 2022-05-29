use crate::args::{
    ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgNamespaceId, ArgUserId, Args,
};
use crate::cmd::Cmd;
use crate::gitlab::GitlabApi;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::{AccessLevel, ConfigFile};
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct AddNamespaceCmd {
    file_name: String,
    gitlab_user_id: u64,
    gitlab_group_id: u64,
    gitlab_url: String,
    gitlab_token: String,
    access_level: AccessLevel,
}

impl Cmd for AddNamespaceCmd {
    type CmdType = AddNamespaceCmd;

    fn add() -> Command<'static> {
        Command::new("add-namespace")
            .alias("an")
            .about("Add a user access to a namespace")
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgNamespaceId::add())
            .arg(ArgUserId::add())
            .arg(ArgAccess::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            file_name: ArgFileName::parse(sub_matches)?,
            gitlab_user_id: ArgUserId::parse(sub_matches)?,
            gitlab_group_id: ArgNamespaceId::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
            access_level: ArgAccess::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> std::io::Result<()> {
        match ConfigFile::read(self.file_name.clone())?.get_version()? {
            Versions::V1 => self.exec_v1(),
        }
    }
}

impl AddNamespaceCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(self.file_name.clone(), self.file_name.clone());
        svc.add_to_namespace(
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.gitlab_group_id,
            self.gitlab_user_id,
            self.access_level,
        )?
        .write_state()
    }
}
