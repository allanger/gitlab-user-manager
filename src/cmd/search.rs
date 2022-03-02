mod groups_cmd;
mod projects_cmd;
mod users_cmd;

use std::io::{Error, ErrorKind};

use clap::{Command, ArgMatches};

use gitlab::Gitlab;

use crate::{
    cmd::Cmd,
    cmd::args::{arg_gitlab_token, arg_gitlab_url},
};

/// Register search cmd
pub(crate) fn add_search_cmd() -> Command<'static> {
    return Command::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg_required_else_help(true)
        .subcommand(projects_cmd::find_projects())
        .subcommand(users_cmd::find_users())
        .subcommand(groups_cmd::find_groups());
}

pub(crate) struct SearchCmd<'a> {
    // search_string: String,
    search_sub: Option<(&'a str, &'a ArgMatches)>,
    gitlab_client: Gitlab,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    // Get gitlab token from flags
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

    // Get search subcommand
    let search_sub = sub_matches.subcommand();

    Ok(SearchCmd {
        search_sub,
        gitlab_client,
    })
}

impl<'a> Cmd<'a> for SearchCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.search_sub {
            Some(("users", sub_matches)) => {
                result = match users_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                };
            }
            Some(("projects", sub_matches)) => {
                result = match projects_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                };
            }
            Some(("groups", sub_matches)) => {
                result = match groups_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                };
            }
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "You should specify what you are looking for, please use help",
                ));
            }
        }
        return result;
    }
}
