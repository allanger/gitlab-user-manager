use std::fs::File;
use clap_complete::{Shell, generate};
use std::io::{Result, Error, ErrorKind};
use std::fs;
use dirs;
use crate::output::out_message::OutMessage;
use crate::cli;

pub(crate) struct GenerateService {
    shell: Shell,
    file_path: String,
}

impl GenerateService {
    pub(crate) fn new(shell: Shell) -> Self {
        Self { 
            shell,
            file_path: "".to_string(),
        }
    }

    pub(crate) fn create_config_dir(&mut self) -> Result<&mut Self>{
        self.file_path = format!("{}/.config/gum/completions", dirs::home_dir().ok_or(Error::new(ErrorKind::NotFound, "Couldn't create a config dir, sorry"))?.to_str().unwrap());
        OutMessage::message_info_with_alias(format!("Creating this dir: {} to save some system stuff", self.file_path).as_str());
        fs::create_dir_all(self.file_path.to_string())?;
        Ok(self)
    }

    pub(crate) fn generate(&self) -> Result<()> {
        let cmd = cli::build();
        let mut file = File::create(format!("{}/_{}", self.file_path, cmd.get_name()))?;
        generate(
            self.shell,
            &mut cmd.clone(),
            &cmd.clone().get_name().to_string(),
            &mut file,
        );
        Ok(())
    }
}
