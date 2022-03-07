use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "file";

pub(crate) struct ArgFileName {
    value: String,
}

impl ArgFileName {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args for ArgFileName {
    type ArgType = ArgFileName;

    fn add() -> Arg<'static> {
        return Arg::new(ARG)
            .short('f')
            .long(ARG)
            .takes_value(true)
            .value_name("FILE_PATH")
            .help("Provide a name of the config file")
            .default_value("gum-config.yaml")
            .global(true);
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        sub_matches
            .value_of(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab token is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(ArgFileName {
                    value: value.to_string(),
                });
            })
    }
}
