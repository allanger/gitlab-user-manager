use std::io::{Error, ErrorKind, Result};

use gitlab::{
    api::{users, Query},
    Gitlab,
};
use tabled::Table;

use crate::{
    gitlab::User,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
};

use super::SearchEntity;

pub(crate) struct Users<'a> {
    gitlab_client: &'a Gitlab,
}

impl<'a> Users<'a> {
    pub fn new(gitlab_client: &'a Gitlab) -> Self {
        Users { gitlab_client }
    }
}

impl<'a> SearchEntity for Users<'a> {
    fn search(&self, query: &str) -> Result<()> {
        let spinner = OutSpinner::spinner_start("Looking for users".to_string());
        let users = match users::Users::builder().search(query).build() {
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
