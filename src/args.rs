pub(crate) mod access_level;
pub(crate) mod dry_run;
pub(crate) mod file_name;
pub(crate) mod gitlab_token;
pub(crate) mod gitlab_url;
pub(crate) mod group_id;
pub(crate) mod project_id;
pub(crate) mod state_destination;
pub(crate) mod state_source;
pub(crate) mod team_name;
pub(crate) mod user_id;
pub(crate) mod write_state;

use clap::{Arg, ArgMatches};
use std::io::Result;

pub(crate) trait Args<'a> {
    type ArgType;
    fn add() -> Arg<'static>;
    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType>;
}
