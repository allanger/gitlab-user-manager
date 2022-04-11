mod access_level;
mod dry_run;
mod file_name;
mod gitlab_token;
mod gitlab_url;
mod group_id;
mod group_list;
mod large_out;
mod namespace_id;
mod no_confirm;
mod project_id;
mod shell;
mod state_destination;
mod state_source;
mod team_name;
mod user_id;
mod write_state;

pub(crate) use self::access_level::ArgAccess;
pub(crate) use self::dry_run::ArgDryRun;
pub(crate) use self::file_name::ArgFileName;
pub(crate) use self::gitlab_token::ArgGitlabToken;
pub(crate) use self::gitlab_url::ArgGitlabUrl;
pub(crate) use self::group_id::ArgGroupId;
pub(crate) use self::group_list::ArgGroupList;
pub(crate) use self::large_out::ArgLargeOut;
pub(crate) use self::namespace_id::ArgNamespaceId;
pub(crate) use self::no_confirm::ArgNoConfirm;
pub(crate) use self::project_id::ArgProjectId;
pub(crate) use self::shell::ArgShell;
pub(crate) use self::state_destination::ArgStateDestination;
pub(crate) use self::state_source::ArgStateSource;
pub(crate) use self::team_name::ArgTeamName;
pub(crate) use self::user_id::ArgUserId;
pub(crate) use self::write_state::ArgWriteState;

use clap::{Arg, ArgMatches};
use std::io::Result;

pub(crate) trait Args {
    type ArgType;
    fn add() -> Arg<'static>;
    fn parse<'b>(sub_matches: &'b ArgMatches) -> Result<Self::ArgType>;
}
