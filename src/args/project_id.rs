use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "project-id";

pub(crate) struct ArgProjectId {
    value: u64,
}

impl ArgProjectId {
    pub(crate) fn value(&self) -> u64 {
        self.value.clone()
    }
}

impl Args for ArgProjectId {
    type ArgType = ArgProjectId;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('i')
            .takes_value(true)
            .value_name("PROJECT_ID")
            .help("Provide the GitLab project ID")
            .default_value("-1")
            .global(true)
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        match sub_matches.value_of_t(ARG) {
            Ok(value) => return Ok(ArgProjectId { value }),
            Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
        };
    }
}
