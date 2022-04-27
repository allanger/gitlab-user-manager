use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::{
    args::{ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgProjectId, ArgUserId, Args},
    gitlab::GitlabActions,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{AccessLevel, ConfigFile, Project},
};
use crate::{cmd::CmdOld, gitlab::GitlabClient};

pub(crate) struct AddProjectCmd {
    file_name: String,
    gitlab_user_id: u64,
    access_level: AccessLevel,
    gitlab_project_id: u64,
    gitlab_client: Gitlab,
}
pub(crate) fn add_add_project_cmd() -> Command<'static> {
    return Command::new("add-project")
        .alias("ap")
        .about("Add user to project")
        .arg(ArgUserId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgAccess::add())
        .arg(ArgProjectId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    // Connect to gitlab
    let gitlab_client: Gitlab =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gitlab_project_id = ArgProjectId::parse(sub_matches)?;
    let access_level = ArgAccess::parse(sub_matches)?;
    let gitlab_user_id = ArgUserId::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(AddProjectCmd {
        access_level,
        gitlab_project_id,
        gitlab_client,
        gitlab_user_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for AddProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the project from Gitlab");

        let project = gitlab.get_project_data_by_id(self.gitlab_project_id)?;

        for user in config_file.config_mut().users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as {}",
                    user.name, project.name, self.access_level,
                ));

                let p = Project {
                    access_level: self.access_level,
                    id: project.id,
                    name: project.name,
                };
                if user.projects.iter().any(|i| i.id == p.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} already has an access to this project: '{}'",
                            user.name, p.name
                        ),
                    ));
                }

                user.projects.extend([p]);
                spinner.spinner_success("Added".to_string());
                break;
            }
        }

        config_file.write(self.file_name.clone())
    }
}
