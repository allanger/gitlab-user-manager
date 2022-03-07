use clap::{Arg, ArgMatches};

use super::Args;

static ARG: &str = "large";
pub(crate) struct ArgLargeOut {
    value: bool,
}

impl ArgLargeOut {
    pub(crate) fn value(&self) -> bool {
        self.value
    }
}

impl Args for ArgLargeOut {
    type ArgType = ArgLargeOut;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .long(ARG)
            .short('l')
            .takes_value(false)
            .help("Display a lot of data")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
        Ok(ArgLargeOut {
            value: sub_matches.is_present(ARG),
        })
    }
}
