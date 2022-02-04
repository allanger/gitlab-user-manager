/// cmd module contains commands and argumets which are being parsed from command line
use clap::{arg, App, Arg};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn init_cmd() -> App<'static> {
    let filename = Arg::new("file_name")
        .short('f')
        .long("file_name")
        .takes_value(true)
        .value_name("FILE_NAME")
        .help("Provide a name of the config file");

    return App::new("init")
        .about("Create a default yaml file in the current directory")
        .arg(filename);
}

/// search cmd should be used to fong Gitlab entities which are being used in gum
pub fn search_cmd() -> App<'static> {
    return App::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .subcommand(find_projects())
        .subcommand(find_users())
        .subcommand(find_groups());
}
fn find_projects() -> App<'static> {
    return App::new("projects")
        .about("Look for GitLab projects")
        .aliases(&["p", "project"])
        .arg(arg!(<PROJECT> "Look for projects"));
}

fn find_users<'a>() -> App<'a> {
    return App::new("users")
        .about("Look for GitLab users")
        .aliases(&["u", "user"])
        .arg(arg!(<USER> "Look for users"));
}

fn find_groups() -> App<'static> {
    return App::new("groups")
        .about("Look for GitLab groups")
        .aliases(&["g", "group"])
        .arg(arg!(<GROUP> "Look for groups"));
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

pub(crate) fn teams_cmd() -> App<'static> {
    // Register command
    return App::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .subcommand(create_team())
        .subcommand(list_teams())
        .subcommand(remove_team())
        .subcommand(add_project_to_team())
        .subcommand(remove_project_from_team());
}

fn create_team() -> App<'static> {
    return App::new("create")
        .alias("c")
        .about("Add a team to the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're creating"));
}

fn list_teams() -> App<'static> {
    return App::new("list")
        .alias("l")
        .about("List teams from config file");
}

fn remove_team() -> App<'static> {
    return App::new("remove")
        .alias("r")
        .about("Remove the team from the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're removing"));
}

fn add_project_to_team() -> App<'static> {
    return App::new("add-project")
        .alias("ap")
        .about("Remove the team from the config file")
        .arg(arg_team_name())
        .arg(arg_access())
        .arg(arg_project_id())
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

fn remove_project_from_team() -> App<'static> {
    return App::new("remove-project")
        .alias("rp")
        .about("Remove a Gitlab project from the team")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_team_name())
        .arg(arg_project_id());
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
