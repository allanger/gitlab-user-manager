use std::io::{Error, ErrorKind};

use clap::{App, ArgMatches};

use gitlab::Gitlab;

use crate::{
    args::{arg_gitlab_token, arg_gitlab_url},
    cmd::Cmd,
};

/// Register search cmd
pub(crate) fn add_search_cmd() -> App<'static> {
    return App::new("search")
        .aliases(&["s", "find"])
        .about("Search for GitLab entities")
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .subcommand(projects_cmd::find_projects())
        .subcommand(users_cmd::find_users())
        .subcommand(groups_cmd::find_groups());
}

pub(crate) struct SearchCmd<'a> {
    // search_string: String,
    search_sub: Option<(&'a str, &'a ArgMatches)>,
    gitlab_client: Gitlab,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    // Get gitlab token from flags
    let gitlab_token = sub_matches.value_of("token").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "gitlab token is not specified",
    ));
    if gitlab_token.is_err() {
        return Err(gitlab_token.err().unwrap());
    }
    // Get gitlab url from flags
    let gitlab_url = sub_matches.value_of("url").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "gitlab url is not specified",
    ));
    if gitlab_url.is_err() {
        return Err(gitlab_token.err().unwrap());
    }

    // Connect to gitlab
    let gitlab_client: Gitlab = match Gitlab::new(
        gitlab_url.unwrap().to_string(),
        gitlab_token.unwrap().to_string(),
    ) {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    // Get search subcommand
    let search_sub = sub_matches.subcommand();

    Ok(SearchCmd {
        search_sub,
        gitlab_client,
    })
}

impl<'a> Cmd<'a> for SearchCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let result;
        match self.search_sub {
            Some(("users", sub_matches)) => {
                result = match users_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                };
            }
            Some(("projects", sub_matches)) => {
                result = match projects_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                };
            }
            Some(("groups", sub_matches)) => {
                result = match groups_cmd::prepare(sub_matches, &self.gitlab_client) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                };
            }
            _ => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "You should specify what you are looking for, please use help",
                ));
            }
        }
        return result;
    }
}

mod users_cmd {
    use std::io::{Error, ErrorKind};

    use clap::{arg, App, ArgMatches};
    use gitlab::{
        api::{users, Query},
        Gitlab,
    };

    use crate::{cmd::Cmd, third_party::gitlab::User};

    pub(crate) fn find_users<'a>() -> App<'a> {
        return App::new("users")
            .about("Look for GitLab users")
            .aliases(&["u", "user"])
            .arg(arg!(<SEARCH> "What you are looking for, mate?"));
    }

    pub(crate) fn prepare<'a>(
        sub_matches: &'a ArgMatches,
        gitlab_client: &'a Gitlab,
    ) -> Result<impl Cmd<'a>, Error> {
        let search_string = sub_matches.value_of("SEARCH").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "whatcha lookin' for, mate?",
        ));
        if search_string.is_err() {
            return Err(search_string.err().unwrap());
        }

        Ok(UsersCmd {
            search_string: search_string.unwrap().to_string(),
            gitlab_client,
        })
    }
    struct UsersCmd<'a> {
        search_string: String,
        gitlab_client: &'a Gitlab,
    }

    impl<'a> Cmd<'a> for UsersCmd<'a> {
        fn exec(&self) -> Result<(), Error> {
            let users = match users::Users::builder().search(&self.search_string).build() {
                Ok(q) => q,
                Err(_err) => return Err(Error::new(ErrorKind::ConnectionRefused, _err)),
            };
            let output: Vec<User> = users.query(self.gitlab_client).unwrap();
            output.iter().enumerate().for_each(|(_, u)| {
                println!("{} | {}", u.name, u.id);
            });
            Ok(())
        }
    }
}

mod projects_cmd {
    use std::io::{Error, ErrorKind};

    use clap::{arg, App, ArgMatches};
    use gitlab::{
        api::{projects, Query},
        Gitlab,
    };

    use crate::{cmd::Cmd, third_party::gitlab::Project};

    pub(crate) fn find_projects<'a>() -> App<'a> {
        return App::new("projects")
            .about("Look for GitLab projects")
            .aliases(&["p", "project"])
            .arg(arg!(<SEARCH> "What you are looking for, mate?"));
    }

    pub(crate) fn prepare<'a>(
        sub_matches: &'a ArgMatches,
        gitlab_client: &'a Gitlab,
    ) -> Result<impl Cmd<'a>, Error> {
        let search_string = sub_matches.value_of("SEARCH").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "whatcha lookin' for, mate?",
        ));
        if search_string.is_err() {
            return Err(search_string.err().unwrap());
        }

        Ok(ProjectsCmd {
            search_string: search_string.unwrap().to_string(),
            gitlab_client,
        })
    }
    struct ProjectsCmd<'a> {
        search_string: String,
        gitlab_client: &'a Gitlab,
    }

    impl<'a> Cmd<'a> for ProjectsCmd<'a> {
        fn exec(&self) -> Result<(), Error> {
            let users = match projects::Projects::builder()
                .search(&self.search_string)
                .build()
            {
                Ok(q) => q,
                Err(_err) => return Err(Error::new(ErrorKind::ConnectionRefused, _err)),
            };
            let output: Vec<Project> = users.query(self.gitlab_client).unwrap();
            output.iter().enumerate().for_each(|(_, u)| {
                println!("{} | {}", u.name, u.id);
            });
            Ok(())
        }
    }
}

mod groups_cmd {
    use std::io::{Error, ErrorKind};

    use clap::{arg, App, ArgMatches};
    use gitlab::{
        api::{groups, Query},
        Gitlab,
    };

    use crate::{cmd::Cmd, third_party::gitlab::Project};

    pub(crate) fn find_groups<'a>() -> App<'a> {
        return App::new("groups")
            .about("Look for GitLab groups")
            .aliases(&["g", "group"])
            .arg(arg!(<SEARCH> "What you are looking for, mate?"));
    }

    pub(crate) fn prepare<'a>(
        sub_matches: &'a ArgMatches,
        gitlab_client: &'a Gitlab,
    ) -> Result<impl Cmd<'a>, Error> {
        let search_string = sub_matches.value_of("SEARCH").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "whatcha lookin' for, mate?",
        ));
        if search_string.is_err() {
            return Err(search_string.err().unwrap());
        }

        Ok(GroupsCmd {
            search_string: search_string.unwrap().to_string(),
            gitlab_client,
        })
    }
    struct GroupsCmd<'a> {
        search_string: String,
        gitlab_client: &'a Gitlab,
    }

    impl<'a> Cmd<'a> for GroupsCmd<'a> {
        fn exec(&self) -> Result<(), Error> {
            let users = match groups::Groups::builder()
                .search(&self.search_string)
                .build()
            {
                Ok(q) => q,
                Err(_err) => return Err(Error::new(ErrorKind::ConnectionRefused, _err)),
            };
            let output: Vec<Project> = users.query(self.gitlab_client).unwrap();
            output.iter().enumerate().for_each(|(_, u)| {
                println!("{} | {}", u.name, u.id);
            });
            Ok(())
        }
    }
}
