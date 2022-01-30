use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Arg};

use crate::{pkg::config, types::types, third_party::{self, gitlab::GitlabActions}};

pub fn teams_pkg(sub_matches: &ArgMatches) -> Option<Error> {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => {
            return create(
                sub_matches
                    .value_of("TEAM_NAME")
                    .expect("Team name is required"),
            )
        }
        Some(("remove", sub_matches)) => {
            return remove(
                sub_matches
                    .value_of("TEAM_NAME")
                    .expect("Team name is required"),
            )
        }
        Some(("list", _)) => {
            return list();
        }
        Some(("add-project", sub_matches)) => add_project(sub_matches),
        _ => return None,
    }
}

fn create(team_name: &str) -> Option<Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Some(_error),
    };

    let new_team = types::Team {
        name: team_name.to_string(),
        projects: None,
    };
    //  TODO: It shouldn't look that bad, I hope
    if config
        .teams
        .as_mut()
        .unwrap()
        .iter()
        .any(|i| i.name == new_team.name)
    {
        return Some(Error::new(
            ErrorKind::AlreadyExists,
            "team with this name already exists",
        ));
    }

    config.teams.as_mut().unwrap().extend([new_team]);

    let _ = match config::write_config(config) {
        Ok(()) => return None,
        Err(_error) => return Some(_error),
    };
}

fn list() -> Option<Error> {
    let config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Some(_error),
    };

    for team in config.teams.unwrap().iter() {
        println!("{}", team.name);
    }
    None
}

fn remove(team_name: &str) -> Option<Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Some(_error),
    };

    //  TODO: It shouldn't look that bad, I hope
    config
        .teams
        .as_mut()
        .unwrap()
        .retain(|t| t.name != team_name);

    let _ = match config::write_config(config) {
        Ok(()) => return None,
        Err(_error) => return Some(_error),
    };
}

fn add_project(sub_matches: &ArgMatches) -> Option<Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Some(_error),
    };
    let g_conn = third_party::gitlab::GitlabConnection{
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string()
    };
    let gitlab = third_party::gitlab::new_gitlab_client(g_conn);
    gitlab.get_project_name_by_id();

    // for team in config.teams.unwrap().iter() {
        // if team.name == "default" {
            // team.projects.as_ref().unwrap().extend([types::Project{
                // access_right: "default".to_string(),
                // id: 1,
                // name: "project".to_string(),
            // }]);
            // break;
        // };
    // }
    None
}
