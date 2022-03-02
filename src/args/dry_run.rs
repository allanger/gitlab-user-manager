use clap::{Arg, ArgMatches};

use super::Args;

static ARG_DRY_RUN: &str = "dry-run";
pub(crate) struct ArgDryRun {
    value: bool,
}

impl ArgDryRun {
    pub(crate) fn value(&self) -> bool {
        self.value
    }
}

impl Args<'_> for ArgDryRun {
    type ArgType = ArgDryRun;

    fn add() -> Arg<'static> {
        Arg::new(ARG_DRY_RUN)
            .long("dry-run")
            .short('d')
            .takes_value(false)
            .help("Use if you wanna see what's gonna happen without actually applying a  new configuration")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
        Ok(ArgDryRun {
            value: sub_matches.is_present(ARG_DRY_RUN),
        })
    }
}
