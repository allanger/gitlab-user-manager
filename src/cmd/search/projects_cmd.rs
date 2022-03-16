use std::io::{Error, ErrorKind};

use clap::{arg, ArgMatches, Command};
use gitlab::{
    api::{projects, Query},
    Gitlab,
};
use tabled::Table;

use crate::{
    cmd::CmdOld,
    gitlab::Project,
    output::{out_message::OutMessage, out_spinner::OutSpinner},
};

pub(crate) fn find_projects<'a>() -> Command<'a> {
    return Command::new("projects")
        .about("Look for GitLab projects")
        .aliases(&["p", "project"])
        .arg(arg!(<SEARCH> "What you are looking for, mate?"));
}

pub(crate) fn prepare<'a>(
    sub_matches: &'_ ArgMatches,
    gitlab_client: &'a Gitlab,
) -> Result<impl CmdOld<'a>, Error> {
    let search_string = sub_matches.value_of("SEARCH").ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::PermissionDenied,
            "whatcha lookin' for, mate?",
        )
    });

    Ok(ProjectsCmd {
        search_string: search_string.unwrap().to_string(),
        gitlab_client,
    })
}
struct ProjectsCmd<'a> {
    search_string: String,
    gitlab_client: &'a Gitlab,
}

impl<'a> CmdOld<'a> for ProjectsCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        let spinner = OutSpinner::spinner_start("Looking for projects".to_string());
        let projects = match projects::Projects::builder()
            .search(&self.search_string)
            .build()
        {
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
