use std::io::{Error, ErrorKind};

use clap::{arg, App, ArgMatches};
use gitlab::Gitlab;

use crate::{
    cmd::{arg_gitlab_token, arg_gitlab_url, Cmd},
    files,
    gitlab::GitlabActions,
    types,
};

pub(crate) fn add_create_cmd() -> App<'static> {
    return App::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

struct CreateCmd {
    gitlab_user_id: u64,
    gitlab_client: Gitlab,
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

    let gitlab_client: Gitlab = match Gitlab::new(
        gitlab_url.unwrap().to_string(),
        gitlab_token.unwrap().to_string(),
    ) {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
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
            Err(_error) => return Err(_error),
        };

        let g = crate::gitlab::new_gitlab_client(self.gitlab_client.to_owned());

        let user = match g.get_user_data_by_id(self.gitlab_user_id) {
            Ok(u) => u,
            Err(_error) => return Err(_error),
        };

        let new_user = types::user::User {
            id: self.gitlab_user_id,
            name: user.name.to_string(),
            ..Default::default()
        };

        if config.users.iter().any(|i| i.id == self.gitlab_user_id) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("user {} is already in the config file", new_user.name),
            ));
        } else {
            config.users.extend([new_user]);
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
