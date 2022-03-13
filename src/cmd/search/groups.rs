use std::io::{Error, ErrorKind, Result};

use gitlab::{
    api::{groups, Query},
    Gitlab,
};
use tabled::Table;

use crate::{
    gitlab::Group,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
};

use super::SearchEntity;

pub(crate) struct Groups<'a> {
    gitlab_client: &'a Gitlab,
}

impl<'a> Groups<'a> {
    pub fn new(gitlab_client: &'a Gitlab) -> Self {
        Groups { gitlab_client }
    }
}

impl<'a> SearchEntity for Groups<'a> {
    fn search(&self, query: &str) -> Result<()> {
        let spinner = OutSpinner::spinner_start("Looking for groups".to_string());
        let groups = match groups::Groups::builder().search(query).build() {
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
