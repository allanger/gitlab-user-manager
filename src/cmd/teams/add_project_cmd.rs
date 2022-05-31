use crate::args::{
    ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgProjectId, ArgTeamName, Args,
};
use crate::cmd::Cmd;
use crate::gitlab::GitlabApi;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::{AccessLevel, ConfigFile};
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct AddProjectCmd {
    file_name: String,
    team_name: String,
    access_level: AccessLevel,
    gitlab_project_id: u64,
    gitlab_token: String,
    gitlab_url: String,
}

impl Cmd for AddProjectCmd {
    type CmdType = AddProjectCmd;

    fn add() -> Command<'static> {
        Command::new("add-project")
            .alias("ap")
            .about("Grant team access to a project")
            .arg(ArgTeamName::add())
            .arg(ArgAccess::add())
            .arg(ArgProjectId::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgFileName::add())
            .arg(ArgGitlabUrl::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            file_name: ArgFileName::parse(sub_matches)?,
            team_name: ArgTeamName::parse(sub_matches)?,
            access_level: ArgAccess::parse(sub_matches)?,
            gitlab_project_id: ArgProjectId::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
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
        let mut svc = v1::TeamsService::new(self.file_name.clone());
        svc.add_to_project(
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.team_name.clone(),
            self.gitlab_project_id,
            self.access_level,
        )?
        .write_state()
    }
}
