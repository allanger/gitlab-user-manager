use super::Args;
use clap::{Arg, ArgMatches};
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "groups";

pub(crate) struct ArgGroupList {
    value: Vec<u64>,
}

impl ArgGroupList {
    /// Get a reference to the arg group list's value.

    /// Get a reference to the arg group list's value.
    pub(crate) fn value(&self) -> &[u64] {
        self.value.as_ref()
    }
}

impl Args<'_> for ArgGroupList {
    type ArgType = ArgGroupList;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('g')
            .long(ARG)
            .takes_value(true)
            .value_name("GROUP_IDS")
            .help("Provide a list of groups to create a snapshot on initializing")
            .global(true)
            .multiple_values(true)
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        let value = match sub_matches.values_of(ARG) {
            Some(v) => v.map(|f| f.parse::<u64>().unwrap()).collect(),
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "You have to provide values for using group-list args",
                ))
            }
        };

        Ok(ArgGroupList { value })
    }
}
