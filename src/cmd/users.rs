mod add_namespace_cmd;
mod add_project_cmd;
mod add_team_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_namespace_cmd;
mod remove_project_cmd;
mod remove_team_cmd;
use self::{
    add_namespace_cmd::AddNamespaceCmd, add_project_cmd::AddProjectCmd, add_team_cmd::AddTeamCmd,
    create_cmd::CreateCmd, list_cmd::ListCmd, remove_cmd::RemoveCmd,
    remove_namespace_cmd::RemoveNamespaceCmd, remove_project_cmd::RemoveProjectCmd,
    remove_team_cmd::RemoveTeamCmd,
};
use super::{Cmd, CmdOld};
use clap::{ArgMatches, Command};
use std::io::Error;

pub(crate) fn add_users_cmd() -> Command {
    return Command::new("users")
        .aliases(&["u"])
        .about("Manage GitLab users")
        .arg_required_else_help(true)
        .subcommand(CreateCmd::add())
        .subcommand(ListCmd::add())
        .subcommand(RemoveCmd::add())
        .subcommand(AddProjectCmd::add())
        .subcommand(RemoveProjectCmd::add())
        .subcommand(AddTeamCmd::add())
        .subcommand(RemoveTeamCmd::add())
        .subcommand(AddNamespaceCmd::add())
        .subcommand(RemoveNamespaceCmd::add());
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
                result = match ListCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-project", sub_matches)) => {
                result = match AddProjectCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-project", sub_matches)) => {
                result = match RemoveProjectCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-team", sub_matches)) => {
                result = match AddTeamCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-team", sub_matches)) => {
                result = match RemoveTeamCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("add-namespace", sub_matches)) => {
                result = match AddNamespaceCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove-namespace", sub_matches)) => {
                result = match RemoveNamespaceCmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            _ => return Ok(()),
        }
        result
    }
}
