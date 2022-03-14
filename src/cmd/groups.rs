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
    add_namespace_cmd::add_add_namespace_cmd, add_project_cmd::add_add_project_cmd,
    create_cmd::add_create_cmd, list_cmd::add_list_cmd, remove_cmd::add_remove_cmd,
    remove_namespace_cmd::add_remove_namespace_cmd, remove_project_cmd::add_remove_project_cmd,
};

use super::CmdOld;

pub(crate) fn add_groups_cmd() -> Command<'static> {
    return Command::new("groups")
        .aliases(&["g", "group"])
        .about("Manage GitLab groups")
        .arg_required_else_help(true)
        .subcommand(add_create_cmd())
        .subcommand(add_list_cmd())
        .subcommand(add_remove_cmd())
        .subcommand(add_add_project_cmd())
        .subcommand(add_remove_project_cmd())
        .subcommand(add_add_namespace_cmd())
        .subcommand(add_remove_namespace_cmd());
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
                result = match create_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(err) => Err(err),
                }
            }
            Some(("remove", sub_matches)) => {
                result = match remove_cmd::prepare(sub_matches) {
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