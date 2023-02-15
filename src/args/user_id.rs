use crate::output::out_message::OutMessage;

use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "user-id";

pub(crate) struct ArgUserId;

impl Args for ArgUserId {
    type ArgType = u64;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('u')
            .long(ARG)
            .value_name("USER_ID")
            .help("Provide the id of the GitLab user")
            .default_value("-1")
            .global(true)
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<u64> {
        sub_matches.get_one::<String>(ARG)
        .ok_or_else(|| {
            let err_msg = "User ID is incorrect";
            OutMessage::message_error(err_msg);
            Error::new(std::io::ErrorKind::InvalidInput, err_msg)
        })
        .and_then(|value| {
            return Ok(value.parse::<u64>().unwrap());
        })
    }
}
