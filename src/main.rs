mod cmd;
mod models;

use std::env;

use clap::{App, AppSettings, Arg};
use cmd::init::init_cmd;
use cmd::search::search_cmd;
use gitlab::api::{self, projects, Query};
use gitlab::Gitlab;
use serde::Deserialize;

use crate::models::models::{Config, Team};

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
}

fn main() {
    let matches = App::new("gum")
        .about("Manage gitlab users even in the free version, beoch!")
        .version("v1.1.1")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("allanger")
        .arg(
            Arg::from_usage("[arg] 'some opt'").env_os(OsStr::new("CLP_TEST_ENV")),
        )
        .subcommand(init_cmd())
        .subcommand(App::new("sync"))
        .subcommand(App::new("user"))
        .subcommand(App::new("team"))
        .subcommand(App::new("projects"))
        .subcommand(App::new("groups"))
        .subcommand(search_cmd())
        .get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let file_name: &str;
            // TODO: Refactor
            if sub_matches.value_of("file_name").is_none() {
                file_name = "gum-config.yaml";
            } else {
                file_name = sub_matches.value_of("file_name").expect("");
            }

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
        Some(("sync", sync_matches)) => {
            println!("sync");
            return;
        }
        Some(("user", sync_matches)) => {
            println!("user");
            return;
        }
        Some(("team", sync_matches)) => {
            println!("team");
            return;
        }
        Some(("projects", sync_matches)) => {
            println!("projects");
            return;
        }
        Some(("groups", sync_matches)) => {
            println!("groups");
            return;
        }
        Some(("search", flags)) => {
            match flags.subcommand() {
                Some(("users", sub)) => {
                    println!("USERS!1");
                }
                None => {
                    println!("NONE");
                }
                Some((&_, _)) => {
                    println!("NONE");
                }
            }
            // TODO: DON"T PUSH THIS TOKEN
            let client = Gitlab::new("gitlab.com", "").unwrap();
            // Some endpoints support pagination. They work on their own or via the `api::paged` function
            // to get further results.
            let pageable_endpoint = projects::Projects::builder()
                .search("optima")
                .build()
                .unwrap();
            // The endpoint on its own is just the first page of results (usually 20 entries).
            let first_page: Vec<Project> = pageable_endpoint.query(&client).unwrap();

            first_page.iter().enumerate().for_each(|(i, x)| {
                println!("Item {} = {}", i, x.name);
            });

            println!("search");
            return;
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}


