use std::ffi::OsStr;

use clap::{arg, App, Arg};

fn find_projects() -> App<'static> {
    return App::new("projects").about("Look for GitLab projects")
    .aliases(&["p", "project"])
    .arg(arg!(<PROJECT> "Look for projects"));
}

fn find_users() -> App<'static> {
    return App::new("users").about("Look for GitLab users")
    .aliases(&["u", "user"])
    .arg(arg!(<USER> "Look for users"));
}

fn find_groups() -> App<'static> {
    return App::new("groups").about("Look for GitLab groups")
    .aliases(&["g", "group"])
    .arg(arg!(<GROUP> "Look for groups"));
}

pub fn search_cmd() -> App<'static> {
    // Define flags
    let arg_gitlab_token = Arg::new("token")
        .short('t')
        .long("token")
        .takes_value(true)
        .value_name("GITLAB_TOKEN")
        .default_value_os(OsStr::new("GITLAB_TOKEN"))
        .help("Provide a name of the config file")
        .env("GITLAB_TOKEN")
        .default_value("GITLAB_TOKEN");

    let arg_gitlab_url = Arg::new("url")
        .short('u')
        .long("url")
        .takes_value(true)
        .value_name("GITLAB_URL")
        .help("Provide the gitlab url if it's not gitlab.com")
        .default_value("gitlab.com");

    return App::new("search")
        .aliases(&["s", "find"])
        .about("Create a default yaml file in the current directory")
        .arg(arg_gitlab_token)
        .arg(arg_gitlab_url)
        .subcommand(find_projects())
        .subcommand(find_users())
        .subcommand(find_groups());
}
