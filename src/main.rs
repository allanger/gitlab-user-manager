mod args;
mod cmd;
mod gitlab;
mod output;
mod service;
mod types;

use clap::Command;
use cmd::{
    groups::{self, add_groups_cmd},
    init::{self, InitCmd},
    search::{self, add_search_cmd},
    sync::{self, add_sync_cmd},
    teams::{self, add_teams_cmd},
    upgrade::{self, add_upgrade_cmd},
    users::{self, add_users_cmd},
    Cmd, CmdOld,
};
use output::{out_extra::OutExtra, out_message::OutMessage};
use std::io::{Error, ErrorKind};
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MESSAGE_OF_THE_DAY: &str = "☮️  Fight war, not wars ☮️";
const NEWS: &[&str] = &["Now I'm able to print some news, so you'll stay informed"];

fn main() {
    OutExtra::welcome_message(MESSAGE_OF_THE_DAY, NEWS);
    let matches = Command::new("gum")
        .about("Manage your GitLab team access in a better way, dude")
        .version(VERSION)
        .author("allanger")
        .arg_required_else_help(true)
        .subcommand(InitCmd::add())
        .subcommand(add_users_cmd())
        .subcommand(add_teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(add_sync_cmd())
        .subcommand(add_upgrade_cmd())
        .subcommand(add_groups_cmd())
        .get_matches();

    let result: Result<(), Error>;

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            result = match InitCmd::prepare(sub_matches) {
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
        Some(("groups", sub_matches)) => {
            result = match groups::prepare(sub_matches) {
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
        Some(("upgrade", sub_matches)) => {
            result = match upgrade::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        _ => result = Err(Error::new(ErrorKind::InvalidInput, "No command provided")),
    }

    match result {
        Err(err) => {
            OutExtra::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutExtra::sum_success("It was fun, wasn't it?");
        }
    }
}
