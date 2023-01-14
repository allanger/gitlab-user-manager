use super::Args;
use crate::output::out_message::OutMessage;
use clap::{Arg, ArgMatches};
use std::io::{Error, Result};

static ARG: &str = "team-name";

pub(crate) struct ArgTeamName;

impl Args for ArgTeamName {
    type ArgType = String;

    fn add() -> Arg {
        Arg::new(ARG)
            .short('n')
            .num_args(1..)
            .value_name("TEAM_NAME")
            .help("Provide a name of the team")
            .default_value("default")
    }

    fn parse<'a>(sub_matches: &'_ ArgMatches) -> Result<String> {
        sub_matches
            .get_one::<String>(ARG)
            .ok_or_else(|| {
                let err_msg = "GitLab token is not specified";
                OutMessage::message_error(err_msg);
                Error::new(std::io::ErrorKind::InvalidInput, err_msg)
            })
            .and_then(|value| {
                return Ok(value.to_string());
            })
    }
}
