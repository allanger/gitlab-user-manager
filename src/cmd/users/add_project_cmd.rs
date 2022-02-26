use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use clap::{arg, Command, ArgMatches};
use gitlab::Gitlab;

use crate::{
    cmd::args::{arg_access, arg_gitlab_token, arg_gitlab_url, arg_project_id},
    files,
    gitlab::GitlabActions,
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
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_access())
        .arg(arg_project_id());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_token = sub_matches.value_of("token").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "gitlab token is not specified",
    ));
    if gitlab_token.is_err() {
        return Err(gitlab_token.err().unwrap());
    }
    // Get gitlab url from flags
    let gitlab_url = sub_matches.value_of("url").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "gitlab url is not specified",
    ));
    if gitlab_url.is_err() {
        return Err(gitlab_token.err().unwrap());
    }

    // Connect to gitlab
    let gitlab_client: Gitlab = match Gitlab::new(
        gitlab_url.unwrap().to_string(),
        gitlab_token.unwrap().to_string(),
    ) {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_project_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let access_level: AccessLevel;
    let access_level_str = sub_matches.value_of("access").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "team name is not specified",
    ));
    if access_level_str.is_err() {
        return Err(access_level_str.err().unwrap());
    }
    access_level = match AccessLevel::from_str(&access_level_str.unwrap().to_string()) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
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
            Err(_error) => return Err(_error),
        };
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());

        let project = match gitlab.get_project_data_by_id(self.gitlab_project_id) {
            Ok(p) => p,
            Err(_error) => return Err(_error),
        };

        for user in config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
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
                break;
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
