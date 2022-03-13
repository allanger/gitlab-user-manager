use crate::args::{gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl, Args};
use clap::{arg, Command};

pub(crate) fn add_search_cmd() -> Command<'static> {
    return Command::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg_required_else_help(true)
        .subcommand(add_search_projects_cmd())
        .subcommand(add_search_users_cmd())
        .subcommand(add_search_groups_cmd());
}

fn add_search_projects_cmd() -> Command<'static> {
    return Command::new("projects")
        .about("Look for GitLab projects")
        .aliases(&["p", "project"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}

fn add_search_users_cmd() -> Command<'static> {
    return Command::new("users")
        .about("Look for GitLab users")
        .aliases(&["u", "user"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}

fn add_search_groups_cmd() -> Command<'static> {
    return Command::new("groups")
        .about("Look for GitLab groups")
        .aliases(&["g", "group"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}
