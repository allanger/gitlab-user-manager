use super::Args;
use crate::{output::out_message::OutMessage, types::v1::access_level::AccessLevel};
use clap::{Arg, ArgMatches};
use std::{
    io::{Error, Result},
    str::FromStr,
};

static ARG: &str = "access";

pub(crate) struct ArgAccess;

impl Args for ArgAccess {
    type ArgType = AccessLevel;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .long(ARG)
            .short('a')
            .takes_value(true)
            .value_name("ACCESS")
            .help("Provide a valid access level")
            .default_value("guest")
            .global(true)
            .possible_values([
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
            .value_of(ARG)
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
