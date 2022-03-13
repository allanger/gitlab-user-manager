mod args;
mod cmd;
mod gitlab;
mod output;
mod types;

use ::gitlab::Gitlab;
use args::{gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl, Args};
use clap::{ArgMatches, Command};
use cmd::{
    init::{self, add_init_cmd},
    search::{commands::add_search_cmd, SearchService},
    sync::{self, add_sync_cmd},
    teams::{self, add_teams_cmd},
    upgrade::{self, add_upgrade_cmd},
    users::{self, add_users_cmd},
    Cmd,
};
use output::{out_message::OutMessage, out_sum::OutSum};
use std::io::{Error, ErrorKind};
use std::process::exit;

static VERSION: &str = "v0.0.6";

fn main() {
    OutMessage::message_empty("\n☮️  Fight war, not wars ☮️\n---");
    let matches = Command::new("gum")
        .about("Manage your GitLab team access in a better way, dude")
        .version(VERSION)
        .author("allanger")
        .arg_required_else_help(true)
        .subcommand(add_init_cmd())
        .subcommand(add_users_cmd())
        .subcommand(add_teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(add_sync_cmd())
        .subcommand(add_upgrade_cmd())
        .get_matches();

    match get_result(matches) {
        Err(err) => {
            OutSum::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutSum::sum_success("Cool, huh?");
        }
    }
}

fn get_result(matches: ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        Some(("init", sub_matches)) => init::prepare(sub_matches).map(|cmd| cmd.exec())?,
        Some(("sync", sub_matches)) => sync::prepare(sub_matches).map(|cmd| cmd.exec())?,
        Some(("users", sub_matches)) => users::prepare(sub_matches).map(|cmd| cmd.exec())?,
        Some(("teams", sub_matches)) => teams::prepare(sub_matches).map(|cmd| cmd.exec())?,
        Some(("search", sub_matches)) => {
            if let Some((entity_name, sub_matches)) = sub_matches.subcommand() {
                let gitlab_token = ArgGitlabToken::parse(sub_matches).map(|arg| arg.value())?;
                let gitlab_url = ArgGitlabUrl::parse(sub_matches).map(|arg| arg.value())?;
                let gitlab_client =
                    Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string()).unwrap();

                let query = sub_matches.value_of("SEARCH").ok_or(Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "whatcha lookin' for, mate?",
                ))?;

                return SearchService::new(&gitlab_client).search(entity_name, query);
            }

            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "You should specify what you are looking for, please use help",
            ));
        }
        Some(("upgrade", sub_matches)) => upgrade::prepare(sub_matches).map(|cmd| cmd.exec())?,
        _ => Err(Error::new(ErrorKind::InvalidInput, "No command provided")),
    }
}
