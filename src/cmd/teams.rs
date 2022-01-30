use clap::{arg, App, Arg};

use super::{arg_project_id, arg_gitlab_token, arg_gitlab_url};

fn arg_team_name() -> Arg<'static> {
    return Arg::new("team-name")
        .short('n')
        .takes_value(true)
        .value_name("TEAM_NAME")
        .help("Provide a name of the team")
        .default_value("default");
}

fn create() -> App<'static> {
    return App::new("create")
        .alias("c")
        .about("Add a team to the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're creating"));
}

fn list() -> App<'static> {
    return App::new("list")
        .alias("l")
        .about("List teams from config file");
}

fn remove() -> App<'static> {
    return App::new("remove")
        .alias("r")
        .about("Remove the team from the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're removing"));
}

fn add_project() -> App<'static> {
    return App::new("add-project")
        .alias("ap")
        .about("Remove the team from the config file")
        .arg(arg_team_name())
        .arg(arg_project_id())
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

fn remove_project() -> App<'static> {
    return App::new("remove-project")
        .alias("rp")
        .about("Remove a Gitlab project from the team")
        .arg(arg_team_name())
        .arg(arg_project_id());
}

pub fn teams_cmd() -> App<'static> {
    // Register command
    return App::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .subcommand(create())
        .subcommand(list())
        .subcommand(remove())
        .subcommand(add_project())
        .subcommand(remove_project());
}
