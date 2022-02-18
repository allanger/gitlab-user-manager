pub(crate) mod init;
pub(crate) mod search;
pub(crate) mod sync;
pub(crate) mod teams;
pub(crate) mod users;

use std::{io::Error, result::Result};

use clap::{App, Arg};

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
        .default_value("guest")
        .global(true);
}

pub(crate) fn arg_team_name() -> Arg<'static> {
    return Arg::new("team-name")
        .short('n')
        .takes_value(true)
        .value_name("TEAM_NAME")
        .help("Provide a name of the team")
        .default_value("default");
}
