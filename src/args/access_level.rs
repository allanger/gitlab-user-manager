use super::Args;
use crate::{output::OutMessage, types::access_level::AccessLevel};
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
        self.value.clone()
    }
}

impl Args<'_> for ArgAccess {
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

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "Access level is not provided";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                match AccessLevel::from_str(value) {
                    Ok(value) => return Ok(ArgAccess { value }),
                    Err(e) => return Err(e),
                };
            })
    }
}
