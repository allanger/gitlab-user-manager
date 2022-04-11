use clap::{Command, ArgMatches};
use crate::cmd::{
    GenerateCmd, groups::{self, add_groups_cmd}, InitCmd, search::{self, add_search_cmd},
    SyncCmd, teams::{self,add_teams_cmd}, upgrade::{self, add_upgrade_cmd}, users::{self, add_users_cmd}, Cmd, CmdOld,
};

use std::io::{Result, Error, ErrorKind};
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn build() -> Command<'static> {
    Command::new("gum")
        .about("Manage your GitLab team access in a better way, dude")
        .version(VERSION)
        .author("allanger")
        .arg_required_else_help(true)
        .subcommand(InitCmd::add())
        .subcommand(GenerateCmd::add())
        .subcommand(add_users_cmd())
        .subcommand(add_teams_cmd())
        .subcommand(add_search_cmd())
        .subcommand(SyncCmd::add())
        .subcommand(add_upgrade_cmd())
        .subcommand(add_groups_cmd())
}

pub(crate) fn exec(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            match InitCmd::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            }
        }
        Some(("generate", sub_matches)) => {
            match GenerateCmd::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            }
        }
        Some(("sync", sub_matches)) => {
            match SyncCmd::prepare(sub_matches) {
                Ok(cmd) => Cmd::exec(&cmd),
                Err(err) => Err(err),
            }
        }
        Some(("users", sub_matches)) => {
            return match users::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("groups", sub_matches)) => {
            return match groups::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        Some(("teams", sub_matches)) => {
            return match teams::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("search", sub_matches)) => {
            return match search::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("upgrade", sub_matches)) => {
            return match upgrade::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        _ => return Err(Error::new(ErrorKind::InvalidInput, "No command provided")),
    }
}
