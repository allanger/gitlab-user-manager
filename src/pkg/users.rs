use std::io::{Error, ErrorKind};

use clap::ArgMatches;

use crate::{
    pkg::config,
    third_party::{self, gitlab::GitlabActions},
    types::types,
};

pub fn users_pkg(sub_matches: &ArgMatches) -> Option<Error> {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => return create(sub_matches),
        Some(("list", _)) => {}
        Some(("remove", sub_matches)) => {}
        Some(("add-project", sub_matches)) => {}
        Some(("add-team", sub_matches)) => {}
        Some(("add-ownership", sub_matches)) => {}
        Some(("remove-project", sub_matches)) => {}
        Some(("remove-team", sub_matches)) => {}
        Some(("remove-ownership", sub_matches)) => {}
        _ => return None,
    };
    None
}

fn create(sub_matches: &ArgMatches) -> Option<Error> {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return Some(_error),
    };
    
    let user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Some(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    let g_conn = third_party::gitlab::GitlabConnection {
        url: sub_matches.value_of("url").unwrap().to_string(),
        token: sub_matches.value_of("token").unwrap().to_string(),
    };

    let gitlab = third_party::gitlab::new_gitlab_client(g_conn.url, g_conn.token);

    let user = match gitlab.get_user_data_by_id(user_id) {
        Ok(u) => u,
        Err(_error) => return Some(_error),
    };

    let new_user = types::User {
        id: user_id,
        name: user.name.to_string(),
        projects: None,
        teams: None,
        ownerships: None,
    };

    //  TODO: It shouldn't look that bad, I hope
    if config
        .users
        .as_mut()
        .unwrap()
        .iter()
        .any(|i| i.name == new_user.name)
    {
        return Some(Error::new(
            ErrorKind::AlreadyExists,
            format!("user {} is already in the config file", new_user.name),
        ));
    }

    config.users.as_mut().unwrap().extend([new_user]);

    let _ = match config::write_config(config) {
        Ok(()) => return None,
        Err(_error) => return Some(_error),
    };
}
fn list(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn remove(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn add_project(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn add_team(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn add_ownership(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn remove_project(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn remove_team(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
fn remove_ownership(sub_matches: &ArgMatches) -> Option<Error> {
    None
}
