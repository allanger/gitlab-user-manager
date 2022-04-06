pub(crate) mod groups;
pub(crate) mod init;
pub(crate) mod search;
pub(crate) mod sync;
pub(crate) mod teams;
pub(crate) mod upgrade;
pub(crate) mod users;

use std::io::Result;

use clap::{ArgMatches, Command};

pub(crate) trait CmdOld<'a> {
    fn exec(&self) -> Result<()>;
}

pub(crate) trait Cmd {
    type CmdType;
    fn add() -> Command<'static>;
    fn prepare(sub_matches: &'_ ArgMatches) -> Result<Self::CmdType>;
    fn exec(&self) -> Result<()>;
}
