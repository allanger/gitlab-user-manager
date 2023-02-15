use crate::output::out_message::OutMessage;

use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "group-id";

pub(crate) struct ArgGroupId;

impl Args for ArgGroupId {
    type ArgType = u64;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('g')
            .long(ARG)
            .value_name("GROUP_ID")
            .help("Provide the id of the GitLab group")
            .default_value("-1")
            .global(true)
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<u64> {
        sub_matches.get_one::<String>(ARG)
        .ok_or_else(|| {
            let err_msg = "Group ID is incorrect";
            OutMessage::message_error(err_msg);
            Error::new(std::io::ErrorKind::InvalidInput, err_msg)
        })
        .and_then(|value| {
            return Ok(value.parse::<u64>().unwrap());
        })
    }
}
