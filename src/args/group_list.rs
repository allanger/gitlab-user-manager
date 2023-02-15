use super::Args;
use clap::{Arg, ArgMatches};
use std::io::Result;

static ARG: &str = "groups";

pub(crate) struct ArgGroupList;

impl Args for ArgGroupList {
    type ArgType = Vec<u64>;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('g')
            .long(ARG)
            .value_name("GROUP_IDS")
            .help("Provide a list of groups to create a snapshot on initializing")
            .global(true)
            .num_args(1..)
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Vec<u64>> {
        let value = match sub_matches.get_many::<String>(ARG) {
            Some(v) => v.map(|f| f.parse::<u64>().unwrap()).collect(),
            None => Vec::new(),
        };
        Ok(value)
    }
}
