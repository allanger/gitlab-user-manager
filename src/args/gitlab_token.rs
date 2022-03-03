use super::Args;
use crate::output::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "token";

pub(crate) struct ArgGitlabToken {
    value: String,
}

impl ArgGitlabToken {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args<'_> for ArgGitlabToken {
    type ArgType = ArgGitlabToken;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .short('t')
            .long(ARG)
            .takes_value(true)
            .value_name("GITLAB_TOKEN")
            .help("Provide a name of the config file")
            .env("GITLAB_TOKEN")
            .default_value("GITLAB_TOKEN")
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
                return Ok(ArgGitlabToken {
                    value: value.to_string(),
                });
            })
    }
}
