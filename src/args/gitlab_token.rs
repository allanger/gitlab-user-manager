use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "token";

pub(crate) struct ArgGitlabToken;

impl Args for ArgGitlabToken {
    type ArgType = String;

    fn add() -> Arg {
        return Arg::new(ARG)
            .short('t')
            .long(ARG)
            .takes_value(true)
            .value_name("GITLAB_TOKEN")
            .help("Provide your GitLab token")
            .env("GITLAB_TOKEN")
            .default_value("GITLAB_TOKEN")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab token is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(value.to_string());
            })
    }
}
