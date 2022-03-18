use clap::{Arg, ArgMatches};

use super::Args;

static ARG: &str = "mock-gitlab";
pub(crate) struct ArgDryRun {
    value: bool,
}

impl ArgDryRun {
    pub(crate) fn value(&self) -> bool {
        self.value
    }
}

impl Args for ArgDryRun {
    type ArgType = ArgDryRun;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .long(ARG)
            .short('d')
            .takes_value(false)
            .help("Use if you wanna see what's gonna happen without actually applying a  new configuration")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
        Ok(ArgDryRun {
            value: sub_matches.is_present(ARG),
        })
    }
}
