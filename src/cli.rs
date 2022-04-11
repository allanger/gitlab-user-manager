use clap::Command;

use crate::cmd::{
    generate::GenerateCmd, groups::add_groups_cmd, init::InitCmd, search::add_search_cmd,
    sync::add_sync_cmd, teams::add_teams_cmd, upgrade::add_upgrade_cmd, users::add_users_cmd, Cmd,
};
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
        .subcommand(add_sync_cmd())
        .subcommand(add_upgrade_cmd())
        .subcommand(add_groups_cmd())
}
