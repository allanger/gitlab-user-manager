use std::io::{Error, ErrorKind};

use clap::{arg, App, ArgMatches};
use gitlab::{
    api::{groups, Query},
    Gitlab,
};

use crate::{cmd::Cmd, gitlab::Project};

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
