use std::io::{Error, ErrorKind};

use clap::ArgMatches;

use crate::{pkg::config, types::types};

pub fn teams_pkg(sub_matches: &ArgMatches) -> Option<Error> {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => {
            return create(
                sub_matches
                    .value_of("TEAM_NAME")
                    .expect("Team name is required"),
            )
        }
        Some(("list", _)) => {
            return list();
        }
        _ => return None,
    };
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
    if config
        .teams
        .as_mut()
        .unwrap()
        .iter()
        .any(|i| i.name == team_name)
    {
        return Some(Error::new(
            ErrorKind::AlreadyExists,
            "team with this name already exists",
        ));
    }

    config.teams.as_mut().unwrap().retain(f)

    config.teams.as_mut().unwrap().extend([new_team]);

    let _ = match config::write_config(config) {
        Ok(()) => return None,
        Err(_error) => return Some(_error),
    };
}
