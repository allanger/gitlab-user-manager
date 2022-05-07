use super::Cmd;
use crate::args::{ArgShell, Args};
use crate::service::GenerateService;
use clap::Command;
use clap_complete::Shell;
use std::io::Result;

pub(crate) struct GenerateCmd {
    shell: Shell,
}

impl Cmd for GenerateCmd {
    type CmdType = GenerateCmd;

    fn add() -> clap::Command<'static> {
        Command::new("generate")
            .about("Generate autocompletion for you shell")
            .arg(ArgShell::add())
    }

    fn prepare(sub_matches: &'_ clap::ArgMatches) -> Result<Self::CmdType> {
        Ok(GenerateCmd {
            shell: ArgShell::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> Result<()> {
        GenerateService::new(self.shell).generate()
    }
}
