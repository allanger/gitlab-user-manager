mod add_project_cmd;
mod create_cmd;
mod list_cmd;
mod remove_cmd;
mod remove_project_cmd;

use std::io::Error;

use clap::{App, ArgMatches};

use self::{
    add_project_cmd::add_add_project_cmd, create_cmd::add_create_cmd, list_cmd::add_list_cmd,
    remove_cmd::add_remove_cmd, remove_project_cmd::add_remove_project_cmd,
};

use super::Cmd;

pub(crate) fn add_teams_cmd() -> App<'static> {
    // Register command
    return App::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .subcommand(add_create_cmd())
        .subcommand(add_list_cmd())
        .subcommand(add_remove_cmd())
        .subcommand(add_add_project_cmd())
        .subcommand(add_remove_project_cmd());
}

pub(crate) struct TeamsCmd<'a> {
    teams_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    Ok(TeamsCmd {
        teams_sub: sub_matches.subcommand(),
    })
}

impl<'a> Cmd<'a> for TeamsCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.teams_sub {
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

            _ => return Ok(()),
        }
        result
    }
}
