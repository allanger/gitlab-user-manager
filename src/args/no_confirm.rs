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

impl Args for ArgNoConfirm {
    type ArgType = ArgNoConfirm;

    fn add() -> Arg {
        Arg::new(ARG)
            .long(ARG)
            .short('y')
            .help("Use if the user shouldn't be prompted to confirm an update")
    }

    fn parse<'b>(sub_matches: &'b ArgMatches) -> std::io::Result<Self::ArgType> {
        Ok(ArgNoConfirm {
            value: sub_matches.contains_id(ARG),
        })
    }
}
