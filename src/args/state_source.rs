use super::Args;
use crate::output::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};
use uuid::Uuid;

static ARG: &str = "state-source";

pub(crate) struct ArgStateSource {
    value: String,
}

impl ArgStateSource {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args<'_> for ArgStateSource {
    type ArgType = ArgStateSource;

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

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "State destination is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(ArgStateSource {
                    value: value.to_string(),
                });
            })
    }
}
