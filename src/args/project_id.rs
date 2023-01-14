use crate::output::out_message::OutMessage;

use super::Args;
use clap::{Arg, ArgMatches, value_parser};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "project-id";

pub(crate) struct ArgProjectId;

impl Args for ArgProjectId {
    type ArgType = u64;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('p')
            .value_name("PROJECT_ID")
            .help("Provide the GitLab project ID")
            .default_value("-1")
            .global(true)
            
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<u64> {
        sub_matches.get_one::<String>(ARG)
        .ok_or_else(|| {
            let err_msg = "Project ID is incorrect";
            OutMessage::message_error(err_msg);
            Error::new(std::io::ErrorKind::InvalidInput, err_msg)
        })
        .and_then(|value| {
            return Ok(value.parse::<u64>().unwrap());
        })
    }
}
