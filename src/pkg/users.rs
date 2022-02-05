use std::io::{Error, ErrorKind};

use clap::ArgMatches;

use crate::{
    pkg::config,
    third_party::{self, gitlab::GitlabActions},
    types::types,
};

pub fn users_pkg(sub_matches: &ArgMatches) -> Result<(), Error> {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => return create(sub_matches),
        Some(("list", _)) => return list(),
        Some(("remove", sub_matches)) => return remove(sub_matches),
        Some(("add-project", sub_matches)) => return add_project(sub_matches),
        Some(("add-team", sub_matches)) => return add_team(sub_matches),
        Some(("add-ownership", sub_matches)) => return add_ownership(sub_matches),
        Some(("remove-project", sub_matches)) => return remove_project(sub_matches),
        Some(("remove-team", sub_matches)) => return remove_team(sub_matches),
        Some(("remove-ownership", sub_matches)) => return remove_ownership(sub_matches),
        _ => return Ok(()),
    };
}

fn create(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let user = match gitlab.get_user_data_by_id(user_id) {
        Ok(u) => u,
        Err(_error) => return Err(_error),
    };

    let new_user = types::User {
        id: user_id,
        name: user.name.to_string(),
        projects: None,
        teams: None,
        ownerships: None,
    };

    match config.users.as_mut() {
        Some(u) => {
            if u.iter().any(|i| i.id == user_id) {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("user {} is already in the config file", new_user.name),
                ));
            }
            u.extend([new_user]);
        }
        // TODO: Refactor this
        None => config.users = Some(vec![new_user]),
    }

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

    for user in config.users.unwrap().iter() {
        println!("{}", user.name);
    }
    Ok(())
}
fn remove(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let user = match gitlab.get_user_data_by_id(user_id) {
        Ok(u) => u,
        Err(_error) => return Err(_error),
    };

    let new_user = types::User {
        id: user_id,
        name: user.name.to_string(),
        projects: None,
        teams: None,
        ownerships: None,
    };

    match config.users.as_mut() {
        Some(u) => {
            if u.iter().any(|i| i.id == user_id) {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("user {} is already in the config file", new_user.name),
                ));
            }
            u.extend([new_user]);
        }
        // TODO: Refactor this
        None => config.users = Some(vec![new_user]),
    }

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}
/// Give a user access to the project
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

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let access = sub_matches.value_of("access").unwrap().to_string();
    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let project = match gitlab.get_project_data_by_id(project_id) {
        Ok(p) => p,
        Err(_error) => return Err(_error),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            let p = types::Project {
                access_right: access,
                id: project_id,
                name: project.name,
            };
            match user.projects.as_mut() {
                Some(v) => {
                    if v.iter().any(|i| i.id == p.id) {
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            format!(
                                "the user {} already has an access to this project: '{}'",
                                user.name, p.name
                            ),
                        ));
                    }

                    user.projects.as_mut().unwrap().extend([p]);
                }
                None => {
                    user.projects = Some(vec![p]);
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

fn add_team(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let team_name = sub_matches.value_of("team-name").unwrap().to_string();

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            match user.teams.as_mut() {
                Some(v) => {
                    if v.iter().any(|i| i == &team_name) {
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            format!(
                                "the user {} already is already in the team: '{}'",
                                user.name, team_name
                            ),
                        ));
                    }

                    user.teams.as_mut().unwrap().extend([team_name]);
                }
                None => {
                    user.teams = Some(vec![team_name]);
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
fn add_ownership(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };
    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };
    // let project_id  = value_t!(sub_matches.value_of("project-id"), u64);
    let group_id: u64 = match sub_matches.value_of_t("group-id") {
        Ok(gid) => gid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };
    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let group = match gitlab.get_group_data_by_id(group_id) {
        Ok(g) => g,
        Err(_error) => return Err(_error),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            let g = types::Ownership {
                url: group.web_url,
                id: group_id,
                name: group.name,
            };
            match user.ownerships.as_mut() {
                Some(v) => {
                    if v.iter().any(|i| i.id == g.id) {
                        return Err(Error::new(
                            ErrorKind::AlreadyExists,
                            format!(
                                "the user {} is the owner of tht group already: '{}'",
                                user.name, g.name
                            ),
                        ));
                    }
                    user.ownerships.as_mut().unwrap().extend([g]);
                }
                None => {
                    user.ownerships = Some(vec![g]);
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

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let project = match gitlab.get_project_data_by_id(project_id) {
        Ok(p) => p,
        Err(_error) => return Err(_error),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            println!("removing {} from {}", project.name, user_id);
            user.projects
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
fn remove_team(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let team_name = sub_matches.value_of("team-name").unwrap().to_string();
    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            println!("removing {} from {}", user.name, team_name);
            user.teams.as_mut().unwrap().retain(|i| i != &team_name);
            break;
        }
    }

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}

fn remove_ownership(sub_matches: &ArgMatches) -> Result<(), Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Err(_error),
    };

    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };

    let group_id: u64 = match sub_matches.value_of_t("group-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let group = match gitlab.get_group_data_by_id(group_id) {
        Ok(g) => g,
        Err(_error) => return Err(_error),
    };

    for user in config.users.as_mut().unwrap().iter_mut() {
        if user.id == user_id {
            println!("removing {} from {}", group.name, user_id);
            user.ownerships
                .as_mut()
                .unwrap()
                .retain(|i| i.id != group_id);
            break;
        }
    }

    let _ = match config::write_config(config) {
        Ok(()) => return Ok(()),
        Err(_error) => return Err(_error),
    };
}
