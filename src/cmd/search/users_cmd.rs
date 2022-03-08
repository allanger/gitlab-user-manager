use std::io::{Error, ErrorKind};

use clap::{arg, ArgMatches, Command};
use gitlab::{
    api::{users, Query},
    Gitlab,
};
use tabled::Table;

use crate::{cmd::Cmd, gitlab::User, output::{out_message::OutMessage, out_spinner::OutSpinner}};

pub(crate) fn find_users<'a>() -> Command<'a> {
    return Command::new("users")
        .about("Look for GitLab users")
        .aliases(&["u", "user"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}

pub(crate) fn prepare<'a>(
    sub_matches: &'_ ArgMatches,
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
        let spinner = OutSpinner::spinner_start("Looking for users".to_string());
        let users = match users::Users::builder().search(&self.search_string).build() {
            Ok(q) => q,
            Err(err) => {
                spinner.spinner_failure(err.to_string());
                return Err(Error::new(ErrorKind::ConnectionRefused, err));
            }
        };
        let output: Vec<User> = users.query(self.gitlab_client).unwrap();
        spinner.spinner_success("That's what we've got for ya".to_string());
        let table = Table::new(&output);
        OutMessage::message_empty(format!("{}", table).as_str());
        Ok(())
    }
}
