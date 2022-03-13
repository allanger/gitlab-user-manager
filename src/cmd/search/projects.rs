use std::io::{Error, ErrorKind, Result};

use gitlab::{
    api::{projects, Query},
    Gitlab,
};
use tabled::Table;

use crate::{
    gitlab::Project,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
};

use super::SearchEntity;

pub(crate) struct Projects<'a> {
    gitlab_client: &'a Gitlab,
}

impl<'a> Projects<'a> {
    pub fn new(gitlab_client: &'a Gitlab) -> Self {
        Projects { gitlab_client }
    }
}

impl<'a> SearchEntity for Projects<'a> {
    fn search(&self, query: &str) -> Result<()> {
        let spinner = OutSpinner::spinner_start("Looking for projects".to_string());
        let projects = match projects::Projects::builder().search(query).build() {
            Ok(q) => q,
            Err(err) => {
                spinner.spinner_failure(err.to_string());
                return Err(Error::new(ErrorKind::ConnectionRefused, err));
            }
        };
        let output: Vec<Project> = projects.query(self.gitlab_client).unwrap();
        spinner.spinner_success("That's what we've got for ya".to_string());
        let table = Table::new(&output);
        OutMessage::message_empty(format!("{}", table).as_str());
        Ok(())
    }
}
