mod cmd;
mod pkg;
mod third_party;
mod types;

use std::process::exit;

use clap::{App, AppSettings};
use cmd::{init::init_cmd, search::search_cmd, sync::sync_cmd, teams::teams_cmd, users::users_cmd};
use gitlab::api::{groups, projects, users, Query};
use gitlab::Gitlab;
use pkg::teams::teams_pkg;
use pkg::users::users_pkg;
use serde::Deserialize;

use crate::pkg::init::init_pkg;

#[derive(Debug, Deserialize)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    id: u64,
    name: String,
}
#[derive(Debug, Deserialize)]
struct Groups {
    id: u64,
    name: String,
}

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
        Some(("projects", _)) => {
            println!("projects");
            return;
        }
        Some(("groups", _)) => {
            println!("groups");
            return;
        }
        Some(("search", sub_matches)) => {
            let token = sub_matches
                .value_of("token")
                .expect("gitlab token is missing");
            let url = sub_matches.value_of("url").expect("gitlab url is missing");
            let client = Gitlab::new(url, token).unwrap();

            match sub_matches.subcommand() {
                Some(("users", sub_matches)) => {
                    let users = users::Users::builder()
                        .search(sub_matches.value_of("USER").expect("required"))
                        .build()
                        .unwrap();
                    let output: Vec<User> = users.query(&client).unwrap();
                    output.iter().enumerate().for_each(|(_, u)| {
                        println!("{} | {}", u.name, u.id);
                    })
                }
                Some(("projects", sub_matches)) => {
                    let projects = projects::Projects::builder()
                        .search(sub_matches.value_of("PROJECT").expect("required"))
                        .build()
                        .unwrap();
                    let output: Vec<Project> = projects.query(&client).unwrap();
                    output.iter().enumerate().for_each(|(_, u)| {
                        println!("{} | {}", u.name, u.id);
                    })
                }
                Some(("groups", sub_matches)) => {
                    let projects = groups::Groups::builder()
                        .search(sub_matches.value_of("GROUP").expect("required"))
                        .build()
                        .unwrap();
                    let output: Vec<Groups> = projects.query(&client).unwrap();
                    output.iter().enumerate().for_each(|(_, u)| {
                        println!("{} | {}", u.name, u.id);
                    })
                }

                None => {
                    eprintln!("You should specify what you are looking for, please use help");
                    exit(1);
                }
                _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
            }
            return;
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
