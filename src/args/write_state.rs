use clap::{Arg, ArgMatches};

use super::Args;

static ARG_DRY_RUN: &str = "write-state";
pub(crate) struct ArgWriteState;

impl Args for ArgWriteState {
    type ArgType = bool;

    fn add() -> Arg {
        Arg::new(ARG_DRY_RUN)
            .long(ARG_DRY_RUN)
            .short('w')
            .num_args(1..)
            .help("Use if you wanna save state in a separate json file")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
        Ok(sub_matches.contains_id(ARG_DRY_RUN))
    }
}
