use clap::{Arg, ArgMatches};

use super::Args;

static ARG: &str = "large";
pub(crate) struct ArgLargeOut;

impl Args for ArgLargeOut {
    type ArgType = bool;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .long(ARG)
            .short('l')
            .takes_value(false)
            .help("Display a lot of data")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<bool> {
        Ok(sub_matches.is_present(ARG))
    }
}
