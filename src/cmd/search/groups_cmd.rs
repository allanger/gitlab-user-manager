use std::io::{Error, ErrorKind};

use clap::{arg, ArgMatches, Command};
use gitlab::{
    api::{groups, Query},
    Gitlab,
};
use tabled::Table;

use crate::{
    cmd::Cmd,
    gitlab::{Group, Project},
    output::{OutMessage, OutSpinner},
};

pub(crate) fn find_groups<'a>() -> Command<'a> {
    return Command::new("groups")
        .about("Look for GitLab groups")
        .aliases(&["g", "group"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}

pub(crate) fn prepare<'a>(
    sub_matches: &'a ArgMatches,
    gitlab_client: &'a Gitlab,
) -> Result<impl Cmd<'a>, Error> {
    let search_string = sub_matches.value_of("SEARCH").ok_or(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Whatcha lookin' for, mate?",
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
        let spinner = OutSpinner::spinner_start("Looking for groups".to_string());
        let groups = match groups::Groups::builder()
            .search(&self.search_string)
            .build()
        {
            Ok(q) => q,
            Err(err) => {
                spinner.spinner_failure(err.to_string());
                return Err(Error::new(ErrorKind::ConnectionRefused, err));
            }
        };
        let output: Vec<Group> = groups.query(self.gitlab_client).unwrap();
        spinner.spinner_success("That's what we've got for ya".to_string());
        let table = Table::new(&output);
        OutMessage::message_empty(format!("{}", table).as_str());
        Ok(())
    }
}
