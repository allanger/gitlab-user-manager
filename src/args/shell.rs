use super::Args;
use clap::{Arg, ArgMatches, ValueEnum};
use clap_complete::Shell;
use std::io::{Error, ErrorKind, Result};

static ARG: &str = "team-name";

pub(crate) struct ArgShell;

impl Args for ArgShell {
    type ArgType = Shell;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('s')
            .value_name("SHELL")
            .value_parser(Shell::value_variants())
            .default_value("zsh")
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Shell> {
        return sub_matches.value_of_t::<Shell>(ARG).or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Provided shell doesn't exist",
        )));
    }
}
