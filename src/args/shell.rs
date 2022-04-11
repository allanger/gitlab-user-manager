use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use clap_complete::Shell;
use std::io::{Error, Result, ErrorKind};

static ARG: &str = "team-name";

pub(crate) struct ArgShell;

impl Args for ArgShell {
    type ArgType = Shell;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('s')
            .takes_value(true)
            .value_name("SHELL")
            .possible_values(Shell::possible_values())
            .default_value("zsh")
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Shell> {
        return sub_matches.value_of_t::<Shell>(ARG).or(Err(Error::new(
            ErrorKind::InvalidInput,
            "Provided shell doesn't exist",
        )));
    }
}
