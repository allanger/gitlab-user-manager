mod groups_cmd;
mod projects_cmd;
mod users_cmd;

use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use gitlab::Gitlab;

use crate::{
    args::{ArgGitlabToken, ArgGitlabUrl, Args},
    cmd::CmdOld,
};

/// Register search cmd
pub(crate) fn add_search_cmd() -> Command {
    return Command::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
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

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    // Connect to gitlab
    let gitlab_client =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    // Get search subcommand
    let search_sub = sub_matches.subcommand();

    Ok(SearchCmd {
        search_sub,
        gitlab_client,
    })
}

impl<'a> CmdOld<'a> for SearchCmd<'a> {
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
        result
    }
}
