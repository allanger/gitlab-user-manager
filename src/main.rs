mod cmd;
mod pkg;
mod srv;
mod third_party;
mod types;

use std::io::Error;
use std::process::exit;

use clap::{App, AppSettings};

use cmd::{init_cmd, search_cmd, sync_cmd, teams_cmd, users_cmd};
use pkg::search::search_pkg;
use pkg::teams::teams_pkg;
use pkg::users::users_pkg;

use crate::srv::srv::{new_srv, Init};

fn main() {
    let matches = App::new("gum")
        .about("Manage gitlab users even in the free version, beoch!")
        .version("v1.1.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("allanger")
        .subcommand(init_cmd())
        .subcommand(users_cmd())
        .subcommand(teams_cmd())
        .subcommand(search_cmd())
        .subcommand(sync_cmd())
        .get_matches();
    let error: Result<(), Error>;
    match matches.subcommand() {
        Some(("init", _)) => {
            error = new_srv().exec("gum-config.yaml".to_string());
        }
        Some(("sync", _)) => {
            println!("sync");
            return;
        }
        Some(("users", sub_matches)) => {
            error = users_pkg(sub_matches);
        }
        Some(("teams", sub_matches)) => {
            error = teams_pkg(sub_matches);
        }
        Some(("search", sub_matches)) => {
            error = search_pkg(sub_matches);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
    match error {
        Ok(()) => println!("cool, huh?"),
        Err(_error) => {
            println!("ERROR: {}", _error);
            exit(1);
        }
    }
}
