use clap::{Arg, ArgMatches};

use super::Args;

static ARG: &str = "large";
pub(crate) struct ArgLargeOut;

impl Args for ArgLargeOut {
    type ArgType = bool;

    fn add() -> Arg {
        Arg::new(ARG)
            .long(ARG)
            .short('l')
            .num_args(0)
            .help("Display a lot of data")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<bool> {
        Ok(sub_matches.contains_id(ARG))
    }
}
