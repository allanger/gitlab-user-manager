use clap::{arg, App};

fn create() -> App<'static> {
    return App::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

fn list() -> App<'static> {
    return App::new("list")
        .alias("l")
        .about("List users from the config file");
}

fn remove() -> App<'static> {
    return App::new("remove")
        .alias("r")
        .about("Remove user from the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

fn add_project() -> App<'static> {
    return App::new("add-project")
        .alias("ap")
        .about("Add user to project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}
fn add_team() -> App<'static> {
    return App::new("add-team")
        .alias("at")
        .about("Add user to the team")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}
fn add_ownership() -> App<'static> {
    return App::new("add-ownership")
        .alias("ao")
        .about("Set the user as the group owner")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

fn remove_project() -> App<'static> {
    return App::new("remove-project")
        .alias("rp")
        .about("Remove user from the project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}
fn remove_team() -> App<'static> {
    return App::new("remove-team")
        .alias("rt")
        .about("Remove a user from the team")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}
fn remove_ownership() -> App<'static> {
    return App::new("remove-ownership")
        .alias("ro")
        .about("Remove an ownership from the user")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

pub fn users_cmd() -> App<'static> {
    // Register command
    return App::new("users")
        .aliases(&["u", "users"])
        .about("Manage GitLab users")
        .subcommand(create())
        .subcommand(list())
        .subcommand(remove())
        .subcommand(add_project())
        .subcommand(add_team())
        .subcommand(add_ownership())
        .subcommand(remove_project())
        .subcommand(remove_team())
        .subcommand(remove_ownership());
}
