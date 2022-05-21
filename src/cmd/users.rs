mod add_namespace_cmd;
mod add_project_cmd;
mod add_team_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_namespace_cmd;
mod remove_project_cmd;
mod remove_team_cmd;

use std::io::Error;

use clap::{ArgMatches, Command};

use self::{
    add_namespace_cmd::add_add_namespace_cmd, add_project_cmd::add_add_project_cmd,
    add_team_cmd::add_add_team_cmd, create_cmd::CreateCmd, list_cmd::add_list_cmd,
    remove_cmd::RemoveCmd, remove_namespace_cmd::add_remove_namespace_cmd,
    remove_project_cmd::add_remove_project_cmd, remove_team_cmd::add_remove_team_cmd,
};

use super::{Cmd, CmdOld};

pub(crate) fn add_users_cmd() -> Command<'static> {
    return Command::new("users")
        .aliases(&["u", "users"])
        .about("Manage GitLab users")
        .arg_required_else_help(true)
        .subcommand(CreateCmd::add())
        .subcommand(add_list_cmd())
        .subcommand(RemoveCmd::add())
        .subcommand(add_add_project_cmd())
        .subcommand(add_remove_project_cmd())
        .subcommand(add_add_team_cmd())
        .subcommand(add_remove_team_cmd())
        .subcommand(add_add_namespace_cmd())
        .subcommand(add_remove_namespace_cmd());
}

pub(crate) struct UsersCmd<'a> {
    users_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'_>, Error> {
    Ok(UsersCmd {
        users_sub: sub_matches.subcommand(),
    })
}

impl<'a> CmdOld<'a> for UsersCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.users_sub {
            Some(("create", sub_matches)) => {
                result = match CreateCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove", sub_matches)) => {
                result = match RemoveCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("list", sub_matches)) => {
                result = match list_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-project", sub_matches)) => {
                result = match add_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-project", sub_matches)) => {
                result = match remove_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-team", sub_matches)) => {
                result = match add_team_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-team", sub_matches)) => {
                result = match remove_team_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-namespace", sub_matches)) => {
                result = match add_namespace_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-namespace", sub_matches)) => {
                result = match remove_namespace_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            _ => return Ok(()),
        }
        result
    }
}
