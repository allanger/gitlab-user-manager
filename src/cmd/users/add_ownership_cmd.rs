use std::io::{Error, ErrorKind};

use clap::{arg, Command, ArgMatches};
use gitlab::Gitlab;

use crate::cmd::Cmd;
use crate::{
    cmd::args::{arg_gitlab_token, arg_gitlab_url, arg_group_id},
    files,
    gitlab::{GitlabActions, GitlabClient},
    types,
};

pub(crate) struct AddOwnershipCmd {
    gitlab_user_id: u64,
    gitlab_group_id: u64,
    gitlab_client: Gitlab,
}
pub(crate) fn add_add_ownership_cmd() -> Command<'static> {
    return Command::new("add-ownership")
        .alias("ao")
        .about("Set the user as the group owner")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_group_id())
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_token = sub_matches
        .value_of("token")
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::PermissionDenied,
                "gitlab token is not specified",
            )
        })
        .unwrap();

    let gitlab_url = sub_matches
        .value_of("url")
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::PermissionDenied,
                "gitlab url is not specified",
            )
        })
        .unwrap();

    // Connect to gitlab
    let gitlab_client: Gitlab = match Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string())
    {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_group_id: u64 = match sub_matches.value_of_t("group-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    Ok(AddOwnershipCmd {
        gitlab_group_id,
        gitlab_client,
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for AddOwnershipCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());

        let group = match gitlab.get_group_data_by_id(self.gitlab_group_id) {
            Ok(p) => p,
            Err(_error) => return Err(_error),
        };

        for user in config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let o = types::ownership::Ownership {
                    id: group.id,
                    name: group.name,
                    url: group.web_url,
                };
                if user.ownerships.iter().any(|i| i.id == o.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} is already owner of this group: '{}'",
                            user.name, o.name
                        ),
                    ));
                }

                user.ownerships.extend([o]);
                break;
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
