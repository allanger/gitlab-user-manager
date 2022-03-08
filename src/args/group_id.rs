use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "group-id";

pub(crate) struct ArgGroupId {
    value: u64,
}

impl ArgGroupId {
    pub(crate) fn value(&self) -> u64 {
        self.value.clone()
    }
}

impl Args for ArgGroupId {
    type ArgType = ArgGroupId;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('i')
            .long(ARG)
            .takes_value(true)
            .value_name("GROUP_ID")
            .help("Provide the id of the GitLab group")
            .default_value("-1")
            .global(true)
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Self> {
        match sub_matches.value_of_t(ARG) {
            Ok(value) => Ok(ArgGroupId { value }),
            Err(err) => Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
        }
    }
}
