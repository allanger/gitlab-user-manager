mod add_project_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_project_cmd;

use std::io::Error;

use clap::{ArgMatches, Command};

use self::{
    add_project_cmd::AddProjectCmd, create_cmd::CreateCmd, list_cmd::ListCmd,
    remove_cmd::RemoveCmd, remove_project_cmd::RemoveProjectCmd,
};

use super::{Cmd, CmdOld};

pub(crate) fn add_teams_cmd() -> Command<'static> {
    // Register command
    return Command::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .arg_required_else_help(true)
        .subcommand(CreateCmd::add())
        .subcommand(ListCmd::add())
        .subcommand(RemoveCmd::add())
        .subcommand(AddProjectCmd::add())
        .subcommand(RemoveProjectCmd::add());
}

pub(crate) struct TeamsCmd<'a> {
    teams_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    Ok(TeamsCmd {
        teams_sub: sub_matches.subcommand(),
    })
}

impl<'a> CmdOld<'a> for TeamsCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.teams_sub {
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

            _ => return Ok(()),
        }
        result
    }
}
