mod args;
mod cmd;
mod gitlab;
mod output;
mod types;

use clap::Command;
use cmd::{
    init::{self, add_init_cmd},
    search::{self, add_search_cmd},
    self_update::{self, add_self_update_cmd},
    sync::{self, add_sync_cmd},
    teams::{self, add_teams_cmd},
    users::{self, add_users_cmd},
    Cmd,
};
use output::{OutMessage, OutSum};
use std::io::{Error, ErrorKind};
use std::process::exit;

fn main() {
    OutMessage::message_empty("\n☮️  Fight war, not wars ☮️\n---");
    let matches = Command::new("gum")
        .about("Manage your GitLab team access in a better way, dude")
        .version("v0.0.5")
        .author("allanger")
        .arg_required_else_help(true)
        .subcommand(add_init_cmd())
        .subcommand(add_users_cmd())
        .subcommand(add_teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(add_sync_cmd())
        .subcommand(add_self_update_cmd())
        .get_matches();

    let result: Result<(), Error>;

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            result = match init::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("sync", sub_matches)) => {
            result = match sync::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("users", sub_matches)) => {
            result = match users::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("teams", sub_matches)) => {
            result = match teams::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("search", sub_matches)) => {
            result = match search::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("self-update", sub_matches)) => {
            result = match self_update::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        _ => result = Err(Error::new(ErrorKind::InvalidInput, "No command provided")),
    }
    match result {
        Err(err) => {
            OutSum::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutSum::sum_success("Cool, huh?");
        }
    }
}
