use crate::args::{ArgFileName, ArgProjectId, ArgTeamName, Args};
use crate::cmd::Cmd;
use crate::service::v1;
use crate::types::common::{Version, Versions};
use crate::types::v1::ConfigFile;
use clap::{ArgMatches, Command};
use std::io::Result;

pub(crate) struct RemoveProjectCmd {
    file_name: String,
    team_name: String,
    gitlab_project_id: u64,
}

impl Cmd for RemoveProjectCmd {
    type CmdType = RemoveProjectCmd;

    fn add() -> Command {
        Command::new("remove-project")
            .alias("rp")
            .about("Remove a Gitlab project from the team")
            .arg(ArgTeamName::add())
            .arg(ArgProjectId::add())
            .arg(ArgFileName::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        Ok(Self {
            team_name: ArgTeamName::parse(sub_matches)?,
            gitlab_project_id: ArgProjectId::parse(sub_matches)?,
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
        let mut svc = v1::TeamsService::new(self.file_name.clone());
        svc.remove_from_project(self.team_name.clone(), self.gitlab_project_id)?
            .write_state()
    }
}
