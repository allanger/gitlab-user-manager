pub(crate) mod access_level;
pub(crate) mod dry_run;
pub(crate) mod file_name;
pub(crate) mod gitlab_token;
pub(crate) mod gitlab_url;
pub(crate) mod group_id;
pub(crate) mod large_out;
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

pub(crate) mod no_confirm {
    use clap::{Arg, ArgMatches};

    use super::Args;

    static ARG: &str = "no-confirm";
    pub(crate) struct ArgNoConfirm {
        value: bool,
    }

    impl ArgNoConfirm {
        pub(crate) fn value(&self) -> bool {
            self.value
        }
    }

    impl Args<'_> for ArgNoConfirm {
        type ArgType = ArgNoConfirm;

        fn add() -> Arg<'static> {
            Arg::new(ARG)
                .long(ARG)
                .short('y')
                .takes_value(false)
                .help("Use if the user shouldn't be prompted to confirm an update")
        }

        fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
            Ok(ArgNoConfirm {
                value: sub_matches.is_present(ARG),
            })
        }
    }
}
