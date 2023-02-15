use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "state-destination";

pub(crate) struct ArgStateDestination;

impl Args for ArgStateDestination {
    type ArgType = String;

    fn add() -> Arg {
        return Arg::new(ARG)
            .long(ARG)
            .num_args(1..)
            .value_name("FILE_PATH")
            .help("Provide a path where you would like to save new state")
            .default_value("/tmp/gum/gum-state.json")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .get_one::<String>(ARG)
            .ok_or_else(|| {
                let err_msg = "State destination is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(value.to_string());
            })
    }
}
