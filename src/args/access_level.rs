use super::Args;
use crate::{output::out_message::OutMessage, types::v1::AccessLevel};
use clap::{Arg, ArgMatches};
use std::{
    io::{Error, Result},
    str::FromStr,
};

static ARG: &str = "access";

pub(crate) struct ArgAccess;

impl Args for ArgAccess {
    type ArgType = AccessLevel;

    fn add() -> Arg {
        Arg::new(ARG)
            .long(ARG)
            .short('a')
            .value_name("ACCESS")
            .help("Provide a valid access level")
            .default_value("guest")
            .global(true)
            .value_parser([
                "guest",
                "reporter",
                "developer",
                "maintainer",
                "owner",
                "admin",
            ])
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<AccessLevel> {
        sub_matches
            .get_one::<String>(ARG)
            .ok_or_else(|| {
                let err_msg = "Access level is not provided";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| match AccessLevel::from_str(value) {
                Ok(value) => Ok(value),
                Err(e) => Err(e),
            })
    }
}
