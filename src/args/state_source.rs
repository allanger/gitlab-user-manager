use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "state-source";

pub(crate) struct ArgStateSource;

impl Args for ArgStateSource {
    type ArgType = String;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .long(ARG)
            .short('s')
            .takes_value(true)
            .value_name("FILE_PATH")
            .help("Provide a path of your state file")
            .default_value("")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .value_of(ARG)
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
