use std::io::Error;

use clap::ArgMatches;
use gitlab::{
    api::{groups, projects, users, Query},
    Gitlab,
};

use crate::third_party::gitlab::{Group, Project, User};

pub fn search_pkg(sub_matches: &ArgMatches) -> Result<(), Error> {
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
            });
            Ok(())
        }
        Some(("projects", sub_matches)) => {
            let projects = projects::Projects::builder()
                .search(sub_matches.value_of("PROJECT").expect("required"))
                .build()
                .unwrap();
            let output: Vec<Project> = projects.query(&client).unwrap();
            output.iter().enumerate().for_each(|(_, u)| {
                println!("{} | {}", u.name, u.id);
            });
            Ok(())
        }
        Some(("groups", sub_matches)) => {
            let projects = groups::Groups::builder()
                .search(sub_matches.value_of("GROUP").expect("required"))
                .build()
                .unwrap();
            let output: Vec<Group> = projects.query(&client).unwrap();
            output.iter().enumerate().for_each(|(_, u)| {
                println!("{} | {}", u.name, u.id);
            });
            Ok(())
        }

        _ => {
            eprintln!("You should specify what you are looking for, please use help");
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "You should specify what you are looking for, please use help",
            ));
        }
    }
}
