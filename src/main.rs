mod cmd;
mod pkg;
mod third_party;
mod types;

use clap::{App, AppSettings};
use cmd::{init::init_cmd, search::search_cmd, sync::sync_cmd, teams::teams_cmd, users::users_cmd};

use pkg::search::search_pkg;
use pkg::teams::teams_pkg;
use pkg::users::users_pkg;

use crate::pkg::init::init_pkg;

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

    match matches.subcommand() {
        Some(("init", _)) => {
            init_pkg();
            return;
        }
        Some(("sync", _)) => {
            println!("sync");
            return;
        }
        Some(("users", sub_matches)) => {
            let err = users_pkg(sub_matches);
            if err.is_some() {
                println!("{}", err.unwrap());
            }
            return;
        }
        Some(("teams", sub_matches)) => {
            let err = teams_pkg(sub_matches);
            if err.is_some() {
                println!("{}", err.unwrap());
            }
            return;
        }
        Some(("search", sub_matches)) => {
            let err = search_pkg(sub_matches);
            if err.is_some() {
                println!("{}", err.unwrap());
            }
            return;
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
