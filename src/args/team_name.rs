use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "team-name";

pub(crate) struct ArgTeamName {
    value: String,
}

impl ArgTeamName {
    pub(crate) fn value(&self) -> String {
        self.value.clone()
    }
}

impl Args<'_> for ArgTeamName {
    type ArgType = ArgTeamName;

    fn add() -> Arg<'static> {
        Arg::new(ARG)
            .short('n')
            .takes_value(true)
            .value_name("TEAM_NAME")
            .help("Provide a name of the team")
            .default_value("default")
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
                return Ok(ArgTeamName {
                    value: value.to_string(),
                });
            })
    }
}
