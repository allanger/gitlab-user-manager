mod add_ownership_cmd;
mod add_project_cmd;
mod add_team_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_ownership_cmd;
mod remove_project_cmd;
mod remove_team_cmd;

use std::io::Error;

use clap::{Command, ArgMatches};

use self::{
    add_ownership_cmd::add_add_ownership_cmd, add_project_cmd::add_add_project_cmd,
    add_team_cmd::add_add_team_cmd, create_cmd::add_create_cmd, list_cmd::add_list_cmd,
    remove_cmd::add_remove_cmd, remove_ownership_cmd::add_remove_ownership_cmd,
    remove_project_cmd::add_remove_project_cmd, remove_team_cmd::add_remove_team_cmd,
};

use super::Cmd;

pub(crate) fn add_users_cmd() -> Command<'static> {
    return Command::new("users")
        .aliases(&["u", "users"])
        .about("Manage GitLab users")
        .subcommand(add_create_cmd())
        .subcommand(add_list_cmd())
        .subcommand(add_remove_cmd())
        .subcommand(add_add_project_cmd())
        .subcommand(add_remove_project_cmd())
        .subcommand(add_add_team_cmd())
        .subcommand(add_remove_team_cmd())
        .subcommand(add_add_ownership_cmd())
        .subcommand(add_remove_ownership_cmd());
}

pub(crate) struct UsersCmd<'a> {
    users_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl Cmd<'_>, Error> {
    Ok(UsersCmd {
        users_sub: sub_matches.subcommand(),
    })
}

impl<'a> Cmd<'a> for UsersCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.users_sub {
            Some(("create", sub_matches)) => {
                result = match create_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove", sub_matches)) => {
                result = match remove_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("list", _)) => {
                result = match list_cmd::prepare() {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("add-project", sub_matches)) => {
                result = match add_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove-project", sub_matches)) => {
                result = match remove_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("add-team", sub_matches)) => {
                result = match add_team_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove-team", sub_matches)) => {
                result = match remove_team_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("add-ownership", sub_matches)) => {
                result = match add_ownership_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove-ownership", sub_matches)) => {
                result = match remove_ownership_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }

            _ => return Ok(()),
        }
        result
    }
}
