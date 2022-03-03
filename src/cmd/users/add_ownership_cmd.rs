use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::group_id::ArgGroupId;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::output::{OutMessage, OutSpinner};
use crate::{
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
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgGroupId::add())
        .arg(ArgUserId::add());
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

    let gitlab_group_id = match ArgGroupId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };

    let gitlab_user_id: u64 = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
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
            Err(err) => return Err(err),
        };
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());

        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");

        let group = match gitlab.get_group_data_by_id(self.gitlab_group_id) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };

        for user in config.users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as owner",
                    user.name, group.name
                ));
                let o = types::ownership::Ownership {
                    id: group.id,
                    name: group.name.to_string(),
                    url: group.web_url.to_string(),
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
                spinner.spinner_success("Added".to_string());
            }
        }
        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
