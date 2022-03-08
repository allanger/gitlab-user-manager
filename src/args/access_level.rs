use super::Args;
use crate::{output::out_message::OutMessage, types::v1::access_level::AccessLevel};
use clap::{Arg, ArgMatches};
use std::{
    io::{Error, Result},
    str::FromStr,
};

static ARG: &str = "access";

pub(crate) struct ArgAccess {
    value: AccessLevel,
}

impl ArgAccess {
    pub(crate) fn value(&self) -> AccessLevel {
        self.value
    }
}

impl Args for ArgAccess {
    type ArgType = ArgAccess;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .short('a')
            .takes_value(true)
            .value_name("ACCESS")
            .help("Provide a valid access level")
            .default_value("guest")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "Access level is not provided";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| match AccessLevel::from_str(value) {
                Ok(value) => Ok(ArgAccess { value }),
                Err(e) => Err(e),
            })
    }
}
