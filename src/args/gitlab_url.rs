use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "url";

pub(crate) struct ArgGitlabUrl {
    value: String,
}

impl ArgGitlabUrl {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args for ArgGitlabUrl {
    type ArgType = ArgGitlabUrl;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .long(ARG)
            .takes_value(true)
            .value_name("GITLAB_URL")
            .help("Provide the gitlab url if it's not gitlab.com")
            .default_value("gitlab.com")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab url is not specified";
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
