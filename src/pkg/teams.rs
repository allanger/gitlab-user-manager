use std::io::{Error, ErrorKind};

use clap::ArgMatches;

use crate::{
    pkg::config,
    third_party::{self, gitlab::GitlabActions},
    types::types,
};

pub fn teams_pkg(sub_matches: &ArgMatches) -> Result<(), Error> {
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
        Some(("remove-project", sub_matches)) => remove_project(sub_matches),
        _ => return Ok(()),
    }
}

fn create(team_name: &str) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
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
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "team with this name already exists",
        ));
    }

    config.teams.as_mut().unwrap().extend([new_team]);

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}

fn list() -> Result<(), Error> {
    let config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    for team in config.teams.unwrap().iter() {
        println!("{}", team.name);
    }
    Ok(())
}

fn remove(team_name: &str) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    //  TODO: It shouldn't look that bad, I hope
    config
        .teams
        .as_mut()
        .unwrap()
        .retain(|t| t.name != team_name);

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}

/// Give a team access to the project
fn add_project(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };
    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };
    // let project_id  = value_t!(sub_matches.value_of("project-id"), u64);
    let project_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let team_name = sub_matches.value_of("team-name").unwrap().to_string();

    let access = sub_matches.value_of("access").unwrap().to_string();
    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let project = match gitlab.get_project_data_by_id(project_id) {
        Ok(p) => p,
        Err(_error) => return Err(_error),
    };

    for team in config.teams.as_mut().unwrap().iter_mut() {
        if team.name == team_name {
            let p = types::Project {
                access_right: access,
                id: project_id,
                name: project.name,
            };
            match team.projects.as_mut() {
                Some(v) => {
                    if v.iter().any(|i| i.id == p.id) {
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            format!(
                                "the team '{}' already has an access to this project: '{}'",
                                team.name, p.name
                            ),
                        ));
                    }

                    team.projects.as_mut().unwrap().extend([p]);
                }
                None => {
                    team.projects = Some(vec![p]);
                }
            }

            break;
        }
    }

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}

/// Remove team access from the project
fn remove_project(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };
    // let project_id  = value_t!(sub_matches.value_of("project-id"), u64);
    let project_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let team_name = sub_matches.value_of("team-name").unwrap().to_string();

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let project = match gitlab.get_project_data_by_id(project_id) {
        Ok(p) => p,
        Err(_error) => return Err(_error),
    };

    for team in config.teams.as_mut().unwrap().iter_mut() {
        if team.name == team_name {
            println!("removing {} from {}", project.name, team_name);
            team.projects
                .as_mut()
                .unwrap()
                .retain(|i| i.id != project_id);
            break;
        }
    }

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}
