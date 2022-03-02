use super::Args;
use crate::output::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "url";

pub(crate) struct ArgGitlabUrl {
    value: String,
}

impl ArgGitlabUrl {
    pub(crate) fn value(&self) -> &str {
        self.value.as_ref()
    }
    pub(crate) fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl Args<'_> for ArgGitlabUrl {
    type ArgType = ArgGitlabUrl;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .short('u')
            .long(ARG)
            .takes_value(true)
            .value_name("GITLAB_URL")
            .help("Provide the gitlab url if it's not gitlab.com")
            .default_value("gitlab.com")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab token is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(ArgGitlabUrl {
                    value: value.to_string(),
                });
            })
    }
}
