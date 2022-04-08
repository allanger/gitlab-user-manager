use super::Cmd;
use crate::args::shell::ArgShell;
use crate::args::Args;
use crate::cli;
use crate::service::generate::GenerateService;
use clap::Command;
use clap_complete::{generate, Shell};
use std::fs;
use std::io::{self, Result};

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
        GenerateService::new(self.shell).create_config_dir()?;
        let cmd = cli::build();
        generate(
            self.shell,
            &mut cmd.clone(),
            &cmd.clone().get_name().to_string(),
            &mut io::stdout(),
        );
        Ok(())
    }
}
