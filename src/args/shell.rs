use crate::output::out_message::OutMessage;

use super::Args;
use clap::{Arg, ArgMatches, ValueEnum, value_parser};
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
            .value_parser(value_parser!(Shell))
            .default_value("zsh")
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<Shell> {
        sub_matches.get_one::<Shell>(ARG)
        .ok_or_else(|| {
            let err_msg = "Group ID is incorrect";
            OutMessage::message_error(err_msg);
            Error::new(std::io::ErrorKind::InvalidInput, err_msg)
        })
        .and_then(|value| {
            return Ok(value);
        }).copied()
    }
}
