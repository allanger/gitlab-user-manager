use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "namespace-id";

pub(crate) struct ArgNamespaceId;

impl Args for ArgNamespaceId {
    type ArgType = u64;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('n')
            .long(ARG)
            .value_name("NAMESPACE_ID")
            .help("Provide the id of the GitLab namespace")
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
