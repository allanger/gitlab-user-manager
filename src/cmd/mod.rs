use clap::Arg;

pub(crate) mod init;
pub(crate) mod search;
pub(crate) mod sync;
pub(crate) mod teams;
pub(crate) mod users;

fn arg_gitlab_token() -> Arg<'static> {
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

fn arg_gitlab_url() -> Arg<'static> {
    return Arg::new("url")
        .short('u')
        .long("url")
        .takes_value(true)
        .value_name("GITLAB_URL")
        .help("Provide the gitlab url if it's not gitlab.com")
        .default_value("gitlab.com")
        .global(true);
}

fn arg_project_id() -> Arg<'static> {
    return Arg::new("project-id")
        .short('i')
        .takes_value(true)
        .value_name("PROJECT_ID")
        .help("Provide the GitLab project ID")
        .default_value("-1")
        .global(true);
}

fn arg_access() -> Arg<'static> {
    return Arg::new("access")
        .short('a')
        .takes_value(true)
        .value_name("ACCESS")
        .help("Provide a valid access level")
        .default_value("guest");
}

fn arg_team_name() -> Arg<'static> {
    return Arg::new("team-name")
        .short('n')
        .takes_value(true)
        .value_name("TEAM_NAME")
        .help("Provide a name of the team")
        .default_value("default");
}
