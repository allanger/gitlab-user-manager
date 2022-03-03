use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::gitlab::GitlabClient;
use crate::output::OutMessage;
use crate::{files, gitlab::GitlabActions, types};

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(ArgUserId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add());
}

struct CreateCmd {
    gitlab_user_id: u64,
    gitlab_client: Gitlab,
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

    let gitlab_client: Gitlab = match Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string())
    {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(CreateCmd {
        gitlab_user_id,
        gitlab_client,
    })
}

impl<'a> Cmd<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the user from Gitlab");
        let user = match gitlab.get_user_data_by_id(self.gitlab_user_id) {
            Ok(u) => u,
            Err(err) => return Err(err),
        };

        let new_user = types::user::User {
            id: self.gitlab_user_id,
            name: user.name.to_string(),
            ..Default::default()
        };

        if config.users.iter().any(|i| i.id == self.gitlab_user_id) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("User {} is already in the config file", new_user.name),
            ));
        } else {
            config.users.extend([new_user]);
            OutMessage::message_info_clean(
                format!("User {} is added to the config", user.name).as_str(),
            );
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
