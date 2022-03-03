use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "user-id";

pub(crate) struct ArgUserId {
    value: u64,
}

impl ArgUserId {
    pub(crate) fn value(&self) -> u64 {
        self.value.clone()
    }
}

impl Args<'_> for ArgUserId {
    type ArgType = ArgUserId;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('u')
            .long(ARG)
            .takes_value(true)
            .value_name("USER_ID")
            .help("Provide the id of the GitLab user")
            .default_value("-1")
            .global(true)
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        match sub_matches.value_of_t(ARG) {
            Ok(value) => return Ok(ArgUserId { value }),
            Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
        };
    }
}
