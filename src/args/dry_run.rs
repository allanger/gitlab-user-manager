use clap::{Arg, ArgMatches, ArgAction};
use std::io::{Result, Error};
use crate::output::out_message::OutMessage;

use super::Args;

static ARG: &str = "dry-run";
pub(crate) struct ArgDryRun;


impl Args for ArgDryRun {
    type ArgType = bool;

    fn add() -> Arg {
        Arg::new(ARG)
            .long(ARG)
            .action(ArgAction::SetTrue)
            .short('d')
            .num_args(0)
            .help("Use if you wanna see what's gonna happen without actually applying a  new configuration")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType> {
        sub_matches.get_one::<bool>(ARG).ok_or_else(|| {
            let err_msg = "Wrong value for the dry-run arg";
            OutMessage::message_error(err_msg);
            Error::new(std::io::ErrorKind::InvalidInput, err_msg)
        })
        .and_then(|value| {
            return Ok(value.clone());
        })
    }
}
