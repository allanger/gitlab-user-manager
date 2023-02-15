use crate::{
    args::{ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgProjectId, ArgUserId, Args},
    cmd::Cmd,
    gitlab::GitlabApi,
    service::v1,
    types::{
        common::{Version, Versions},
        v1::{AccessLevel, ConfigFile},
    },
};
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct AddProjectCmd {
    file_name: String,
    gitlab_user_id: u64,
    access_level: AccessLevel,
    gitlab_project_id: u64,
    gitlab_url: String,
    gitlab_token: String,
}

impl Cmd for AddProjectCmd {
    type CmdType = AddProjectCmd;

    fn add() -> Command {
        Command::new("add-project")
            .alias("ap")
            .about("Add user to project")
            .arg(ArgUserId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgAccess::add())
            .arg(ArgProjectId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            file_name: ArgFileName::parse(sub_matches)?,
            gitlab_user_id: ArgUserId::parse(sub_matches)?,
            gitlab_project_id: ArgProjectId::parse(sub_matches)?,
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

impl AddProjectCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(self.file_name.clone());
        svc.add_to_project(
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.gitlab_project_id,
            self.gitlab_user_id,
            self.access_level,
        )?
        .write_state()
    }
}
