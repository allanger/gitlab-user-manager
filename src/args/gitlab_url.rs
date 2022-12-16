use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "url";

pub(crate) struct ArgGitlabUrl;

impl Args for ArgGitlabUrl {
    type ArgType = String;

    fn add() -> Arg {
        return Arg::new(ARG)
            .long(ARG)
            .value_name("GITLAB_URL")
            .help("Provide the gitlab url if it's not gitlab.com")
            .default_value("gitlab.com")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab url is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(value.to_string());
            })
    }
}
