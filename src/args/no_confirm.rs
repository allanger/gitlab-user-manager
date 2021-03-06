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
