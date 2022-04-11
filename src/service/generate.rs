use crate::cli;
use crate::output::out_message::OutMessage;
use clap_complete::{generate, Shell};
use std::fs::File;
use std::io::Result;

pub(crate) struct GenerateService {
    shell: Shell,
}

impl GenerateService {
    pub(crate) fn new(shell: Shell) -> Self {
        Self { shell }
    }

    pub(crate) fn generate(&self) -> Result<()> {
        let cmd = cli::build();
        let mut file = File::create(format!("_{}", cmd.get_name()))?;
        generate(
            self.shell,
            &mut cmd.clone(),
            &cmd.clone().get_name().to_string(),
            &mut file,
        );
        OutMessage::message_info_with_alias(
            format!(
                "The file _{} is generated, put it in your $FPATH to make completions work",
                cmd.get_name()
            )
            .as_str(),
        );
        Ok(())
    }
}
