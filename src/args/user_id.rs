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
        match sub_matches.value_of_t(ARG) {
            Ok(value) => Ok(value),
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
        }
    }
}
