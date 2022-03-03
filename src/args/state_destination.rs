use super::Args;
use crate::output::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "state-destination";

pub(crate) struct ArgStateDestination {
    value: String,
}

impl ArgStateDestination {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args<'_> for ArgStateDestination {
    type ArgType = ArgStateDestination;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .long(ARG)
            .takes_value(true)
            .value_name("FILE_PATH")
            .help("Provide a path where you would like to save new state")
            .default_value("/tmp/gum/gum-state.json")
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
                return Ok(ArgStateDestination {
                    value: value.to_string(),
                });
            })
    }
}
