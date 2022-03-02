pub(crate) mod dry_run;
pub(crate) mod gitlab_token;
pub(crate) mod gitlab_url;
pub(crate) mod group_id;
pub(crate) mod user_id;

use clap::{Arg, ArgMatches};
use std::io::Result;

pub(crate) trait Args<'a> {
    type ArgType;
    fn add() -> Arg<'static>;
    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType>;
}
