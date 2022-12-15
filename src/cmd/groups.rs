mod add_namespace_cmd;
mod add_project_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_namespace_cmd;
mod remove_project_cmd;

use std::io::Error;

use clap::{ArgMatches, Command};

use self::{
    add_namespace_cmd::AddNamespaceCmd, add_project_cmd::AddProjectCmd,
    create_cmd::CreateCmd, list_cmd::ListCmd, remove_cmd::RemoveCmd,
    remove_namespace_cmd::RemoveNamespaceCmd, remove_project_cmd::RemoveProjectCmd,
};

use super::{Cmd, CmdOld};

pub(crate) fn add_groups_cmd() -> Command {
    return Command::new("groups")
        .aliases(&["g", "group"])
        .about("Manage GitLab groups")
        .arg_required_else_help(true)
        .subcommand(CreateCmd::add())
        .subcommand(ListCmd::add())
        .subcommand(RemoveCmd::add())
        .subcommand(AddProjectCmd::add())
        .subcommand(RemoveProjectCmd::add())
        .subcommand(AddNamespaceCmd::add())
        .subcommand(RemoveNamespaceCmd::add());
}

pub(crate) struct UsersCmd<'a> {
    groups_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'_>, Error> {
    Ok(UsersCmd {
        groups_sub: sub_matches.subcommand(),
    })
}

impl<'a> CmdOld<'a> for UsersCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.groups_sub {
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
