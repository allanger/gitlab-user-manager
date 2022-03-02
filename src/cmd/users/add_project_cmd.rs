use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use clap::{arg, ArgMatches, Command};
use gitlab::Gitlab;

use crate::{
    args::{
        access_level::ArgAccess, gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl,
        project_id::ArgProjectId, user_id::ArgUserId, Args,
    },
    files,
    gitlab::GitlabActions,
    output::{OutMessage, OutSpinner},
    types::{self, access_level::AccessLevel},
};
use crate::{cmd::Cmd, gitlab::GitlabClient};

pub(crate) struct AddProjectCmd {
    gitlab_user_id: u64,
    access_level: AccessLevel,
    gitlab_project_id: u64,
    gitlab_client: Gitlab,
}
pub(crate) fn add_add_project_cmd() -> Command<'static> {
    return Command::new("add-project")
        .alias("ap")
        .about("Add user to project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgAccess::add())
        .arg(ArgProjectId::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_token = match ArgGitlabToken::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let gitlab_url = match ArgGitlabUrl::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    // Connect to gitlab
    let gitlab_client: Gitlab = match Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string())
    {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_project_id: u64 = match ArgProjectId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let access_level = match ArgAccess::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(e) => return Err(e),
    };

    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(AddProjectCmd {
        access_level,
        gitlab_project_id,
        gitlab_client,
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for AddProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the project from Gitlab");

        let project = match gitlab.get_project_data_by_id(self.gitlab_project_id) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };

        for user in config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as {}",
                    user.name, project.name, self.access_level,
                ));

                let p = types::project::Project {
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

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
