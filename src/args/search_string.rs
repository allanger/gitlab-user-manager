use super::Args;
use crate::output::OutMessage;
use clap::{arg, Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "search";

pub(crate) struct ArgSearchString {
    value: String,
}

impl ArgSearchString {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args<'_> for ArgSearchString {
    type ArgType = ArgSearchString;

    fn add() -> Arg<'static> {
        arg!(<SEARCH> "What you are looking for, mate?")
    }

    fn parse<'a>(sub_matches: &'a ArgMatches) -> Result<Self> {
        sub_matches
            .value_of("SEARCH")
            .ok_or_else(|| {
                let err_msg = "Search string is not provided";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(ArgSearchString {
                    value: value.to_string(),
                });
            })
    }
}
