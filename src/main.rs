mod cmd;
mod files;
mod gitlab;
mod output;
pub mod types;

use clap::Command;
use cmd::{
    init::{self, add_init_cmd},
    search::{self, add_search_cmd},
    sync::{self, add_sync_cmd},
    teams::{self, add_teams_cmd},
    users::{self, add_users_cmd},
    Cmd,
};
use std::process::exit;
use std::{io::Error, thread::sleep, time::Duration};

fn main() {
    let matches = Command::new("gum")
        .about("Manage gitlab users even in the free version, beoch!")
        .version("v0.0.1")
        .author("allanger")
        .subcommand(add_init_cmd())
        .subcommand(add_users_cmd())
        .subcommand(add_teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(add_sync_cmd())
        .get_matches();
    let result: Result<(), Error>;

    match matches.subcommand() {
        Some(("init", _)) => {
            result = match init::prepare() {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        Some(("sync", sub_matches)) => {
            result = match sync::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        Some(("users", sub_matches)) => {
            result = match users::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        Some(("teams", sub_matches)) => {
            result = match teams::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        Some(("search", sub_matches)) => {
            result = match search::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
    match result {
        Ok(()) => println!("cool, huh?"),
        Err(_error) => {
            println!("ERROR: {}", _error);
            exit(1);
        }
    }
}
