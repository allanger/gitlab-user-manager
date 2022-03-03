pub(crate) mod access_level;
pub(crate) mod dry_run;
pub(crate) mod gitlab_token;
pub(crate) mod gitlab_url;
pub(crate) mod group_id;
pub(crate) mod project_id;
pub(crate) mod team_name;
pub(crate) mod user_id;

use clap::{Arg, ArgMatches};
use std::io::Result;

pub(crate) trait Args<'a> {
    type ArgType;
    fn add() -> Arg<'static>;
    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType>;
}

pub(crate) mod search_string {
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
}
