use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "project-id";

pub(crate) struct ArgProjectId;

impl Args for ArgProjectId {
    type ArgType = u64;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('p')
            .takes_value(true)
            .value_name("PROJECT_ID")
            .help("Provide the GitLab project ID")
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
