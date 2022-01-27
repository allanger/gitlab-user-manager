use clap::{arg, App};

use super::{arg_gitlab_token, arg_gitlab_url};

fn find_projects() ->   App<'static> {
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
    return App::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .subcommand(find_projects())
        .subcommand(find_users())
        .subcommand(find_groups());
}
