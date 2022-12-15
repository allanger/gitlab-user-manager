use clap::{Arg, ArgMatches};
use std::io::Result;
use super::Args;

static ARG: &str = "dry-run";
pub(crate) struct ArgDryRun;


impl Args for ArgDryRun {
    type ArgType = bool;

    fn add() -> Arg {
        Arg::new(ARG)
            .long(ARG)
            .short('d')
            .takes_value(false)
            .help("Use if you wanna see what's gonna happen without actually applying a  new configuration")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType> {
        Ok(sub_matches.is_present(ARG))
    }
}
