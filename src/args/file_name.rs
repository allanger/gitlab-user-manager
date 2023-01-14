use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches, ValueHint};
use std::io::{Error, Result};

static ARG: &str = "file";

pub(crate) struct ArgFileName;

impl Args for ArgFileName {
    type ArgType = String;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('f')
            .long(ARG)
            .value_name("FILE_PATH")
            .help("Provide a name of the config file")
            .default_value("gum-config.yaml")
            .global(true)
            .value_hint(ValueHint::AnyPath)
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .get_one::<String>(ARG)
            .ok_or_else(|| {
                let err_msg = "File is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(value.to_string());
            })
    }
}
