use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "group-id";

pub(crate) struct ArgGroupId;

impl Args for ArgGroupId {
    type ArgType = u64;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('g')
            .long(ARG)
            .takes_value(true)
            .value_name("GROUP_ID")
            .help("Provide the id of the GitLab group")
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
