mod cmd;
mod types;
mod pkg;

use std::process::exit;

use clap::{App, AppSettings};
use cmd::{sync::sync_cmd, init::init_cmd, search::search_cmd, teams::teams_cmd, users::users_cmd};
use gitlab::api::{groups, projects, users, Query};
use gitlab::Gitlab;
use pkg::teams::teams_pkg;
use serde::Deserialize;

use crate::types::types::{Config, Team};

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
}
#[derive(Debug, Deserialize)]
struct Groups {
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
            let file_name = "gum-config.yaml";
            println!("Initializing gum config {:?}", file_name);

            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_name)
                .expect("Couldn't open file");
            // Create default empty config
            let new_config = Config {
                teams: Some(vec![Team {
                    name: "default".to_string(),
                    projects: None,
                }]),
                users: None,
            };
            // Write to file
            serde_yaml::to_writer(f, &new_config).unwrap();

            return;
        }
        Some(("sync", _)) => {
            println!("sync");
            return;
        }
        Some(("user", _)) => {
            println!("user");
            return;
        }
        Some(("teams", sub_matches)) => {
            teams_pkg(sub_matches);
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
                        println!("{}", u.name);
                    })
                }
                Some(("projects", sub_matches)) => {
                    let projects = projects::Projects::builder()
                        .search(sub_matches.value_of("PROJECT").expect("required"))
                        .build()
                        .unwrap();
                    let output: Vec<Project> = projects.query(&client).unwrap();
                    output.iter().enumerate().for_each(|(_, u)| {
                        println!("{}", u.name);
                    })
                }
                Some(("groups", sub_matches)) => {
                    let projects = groups::Groups::builder()
                        .search(sub_matches.value_of("GROUP").expect("required"))
                        .build()
                        .unwrap();
                    let output: Vec<Groups> = projects.query(&client).unwrap();
                    output.iter().enumerate().for_each(|(_, u)| {
                        println!("{}", u.name);
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
