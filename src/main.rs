mod args;
mod cmd;
mod pkg;
mod srv;
mod third_party;
mod types;

use std::io::Error;
use std::process::exit;

use clap::{App, AppSettings};

use cmd::{
    init, init_cmd,
    search::{self, add_search_cmd},
    sync_cmd, teams_cmd, users_cmd, Cmd,
};
use pkg::teams::teams_pkg;
use pkg::users::users_pkg;

use crate::srv::srv::{new_srv, Init};

fn main() {
    let matches = App::new("gum")
        .setting(AppSettings::ArgRequiredElseHelp)
        .about("Manage gitlab users even in the free version, beoch!")
        .version("v1.1.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("allanger")
        .subcommand(init_cmd())
        .subcommand(users_cmd())
        .subcommand(teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(sync_cmd())
        .get_matches();
    let result: Result<(), Error>;

    match matches.subcommand() {
        Some(("init", _)) => {
            result = match init::prepare() {
                Ok(cmd) => cmd.exec(),
                Err(_error) => Err(_error),
            };
        }
        Some(("sync", _)) => {
            println!("sync");
            return;
        }
        Some(("users", sub_matches)) => {
            result = users_pkg(sub_matches);
        }
        Some(("teams", sub_matches)) => {
            result = teams_pkg(sub_matches);
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
