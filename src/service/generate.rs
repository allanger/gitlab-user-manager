use clap_complete::{Shell, generate};
use std::io::{Result, Error, ErrorKind};
use std::fs;
use dirs;
use crate::output::out_message::OutMessage;
use crate::cli;
pub(crate) struct GenerateService {
    shell: Shell,
}

impl GenerateService {
    pub(crate) fn new(shell: Shell) -> Self {
        Self { shell }
    }

    pub(crate) fn create_config_dir(&self) -> Result<()>{
        let config_dir = format!("{}/.config/gum", dirs::home_dir().ok_or(Error::new(ErrorKind::NotFound, "Couldn't create a config dir, sorry"))?.to_str().unwrap());
        OutMessage::message_info_with_alias(format!("Creating this dir: {} to save some system stuff", config_dir).as_str());
        fs::create_dir_all(config_dir)?;
        Ok(())
    }

    pub(crate) fn generate(&self) -> Result<()> {
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
