pub(crate) mod init;
pub(crate) mod search;
pub(crate)mod teams;
pub(crate) mod users;

use std::{io::Error, result::Result};

/// cmd module contains commands and arguments which are being parsed from command line
use clap::{arg, App, Arg};

pub(crate) trait Cmd<'a> {
    fn exec(&self) -> Result<(), Error>;
}
/// sync cmd should be used to sync gum-config with Gitlab and generate or update the state file
pub fn sync_cmd() -> App<'static> {
    let dry_run = Arg::new("dry_run")
        .short('d')
        .takes_value(true)
        .value_name("DRY_RUN")
        .default_value("false")
        .help("Use if you wanna see what's gonna happen without applying new configuration");

    // Register command
    return App::new("sync")
        .about("Sync your config file with GitLab and generate the state file")
        .arg(dry_run);
}

fn arg_group_id() -> Arg<'static> {
    return Arg::new("group-id")
        .short('i')
        .long("group-id")
        .takes_value(true)
        .value_name("GROUP_ID")
        .help("Provide the id for the group")
        .default_value("-1")
        .global(true);
}

fn create_user() -> App<'static> {
    return App::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

fn list_users() -> App<'static> {
    return App::new("list")
        .alias("l")
        .about("List users from the config file");
}

fn remove_user() -> App<'static> {
    return App::new("remove")
        .alias("r")
        .about("Remove user from the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

fn add_user_to_project() -> App<'static> {
    return App::new("add-project")
        .alias("ap")
        .about("Add user to project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_access())
        .arg(arg_project_id());
}

fn add_user_to_team() -> App<'static> {
    return App::new("add-team")
        .alias("at")
        .about("Add user to the team")
        .arg(arg_team_name())
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}
fn add_ownership_to_user() -> App<'static> {
    return App::new("add-ownership")
        .alias("ao")
        .about("Set the user as the group owner")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_group_id())
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

fn remove_user_from_project() -> App<'static> {
    return App::new("remove-project")
        .alias("rp")
        .about("Remove user from the project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_project_id());
}
fn remove_user_from_team() -> App<'static> {
    return App::new("remove-team")
        .alias("rt")
        .about("Remove a user from the team")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_team_name());
}
fn remove_ownership_from_user() -> App<'static> {
    return App::new("remove-ownership")
        .alias("ro")
        .about("Remove an ownership from the user")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_group_id());
}

pub(crate) fn users_cmd() -> App<'static> {
    // Register command
    return App::new("users")
        .aliases(&["u", "users"])
        .about("Manage GitLab users")
        .subcommand(create_user())
        .subcommand(list_users())
        .subcommand(remove_user())
        .subcommand(add_user_to_project())
        .subcommand(add_user_to_team())
        .subcommand(add_ownership_to_user())
        .subcommand(remove_user_from_project())
        .subcommand(remove_user_from_team())
        .subcommand(remove_ownership_from_user());
}

pub(crate) fn arg_gitlab_token() -> Arg<'static> {
    return Arg::new("token")
        .short('t')
        .long("token")
        .takes_value(true)
        .value_name("GITLAB_TOKEN")
        .help("Provide a name of the config file")
        .env("GITLAB_TOKEN")
        .default_value("GITLAB_TOKEN")
        .global(true);
}

pub(crate) fn arg_gitlab_url() -> Arg<'static> {
    return Arg::new("url")
        .short('u')
        .long("url")
        .takes_value(true)
        .value_name("GITLAB_URL")
        .help("Provide the gitlab url if it's not gitlab.com")
        .default_value("gitlab.com")
        .global(true);
}

pub(crate) fn arg_project_id() -> Arg<'static> {
    return Arg::new("project-id")
        .short('i')
        .takes_value(true)
        .value_name("PROJECT_ID")
        .help("Provide the GitLab project ID")
        .default_value("-1")
        .global(true);
}

pub(crate) fn arg_access() -> Arg<'static> {
    return Arg::new("access")
        .short('a')
        .takes_value(true)
        .value_name("ACCESS")
        .help("Provide a valid access level")
        .default_value("guest");
}

pub(crate) fn arg_team_name() -> Arg<'static> {
    return Arg::new("team-name")
        .short('n')
        .takes_value(true)
        .value_name("TEAM_NAME")
        .help("Provide a name of the team")
        .default_value("default");
}
