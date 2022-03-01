use std::io::{Error, ErrorKind};

use clap::Command;

use crate::{cmd::Cmd, output::OutMessage, types::config::Config};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_init_cmd() -> Command<'static> {
    return Command::new("init").about("Create a default yaml file in the current directory");
}

pub(crate) struct InitCmd;

pub(crate) fn prepare<'a>() -> Result<impl Cmd<'a>, Error> {
    Ok(InitCmd)
}

impl<'a> Cmd<'a> for InitCmd {
    fn exec(&self) -> Result<(), Error> {
        let f = "gum-config.yaml";

        let file = match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(f)
        {
            Ok(file) => file,
            Err(err) => {
                return match err.kind() {
                    ErrorKind::AlreadyExists => {
                        return Err(Error::new(
                            err.kind(),
                            "config file already exists in specified directory",
                        ))
                    }
                    _ => Err(Error::new(ErrorKind::AlreadyExists, err)),
                }
            }
        };

        let new_config: Config = Default::default();

        serde_yaml::to_writer(file, &new_config).unwrap();
        OutMessage::message_empty("Config file is generated, check it out\n $ cat gum-config.yaml");
        Ok(())
    }
}
