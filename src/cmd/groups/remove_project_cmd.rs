use std::io::Error;

use clap::{ArgMatches, Command};

use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgGroupId, ArgProjectId, Args};
use crate::cmd::CmdOld;
use crate::output::out_message::OutMessage;
use crate::types::v1::ConfigFile;

pub(crate) struct RemoveProjectCmd {
    gitlab_group_id: u64,
    gitlab_project_id: u64,
    file_name: String,
}
pub(crate) fn add_remove_project_cmd() -> Command<'static> {
    return Command::new("remove-project")
        .alias("rp")
        .about("Remove user from the project")
        .arg(ArgGroupId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgProjectId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_project_id: u64 = ArgProjectId::parse(sub_matches)?;
    let gitlab_group_id = ArgGroupId::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(RemoveProjectCmd {
        gitlab_project_id,
        gitlab_group_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        for g in config_file.config_mut().groups.iter_mut() {
            if g.id == self.gitlab_group_id {
                for (i, p) in g.projects.iter().enumerate() {
                    if p.id == self.gitlab_project_id {
                        OutMessage::message_info_clean(
                            format!("removing user {} from project {}", g.name, p.name).as_str(),
                        );

                        g.projects.remove(i);
                        break;
                    }
                }
            }
        }

        config_file.write(self.file_name.clone())
    }
}
