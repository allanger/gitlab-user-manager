use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, Args},
    cmd::Cmd,
    output::OutMessage,
    types::v1::config_file::ConfigFile,
};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_init_cmd() -> Command<'static> {
    return Command::new("init")
        .about("Create a default yaml file in the current directory")
        .arg(ArgFileName::add());
}

pub(crate) struct InitCmd {
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(InitCmd { file_name })
}

impl<'a> Cmd<'a> for InitCmd {
    fn exec(&self) -> Result<(), Error> {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.file_name.clone())
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

        let new_config: ConfigFile = Default::default();
        match new_config.write(self.file_name.clone()) {
            Ok(_) => {
                OutMessage::message_empty(
                    format!(
                        "Config file is generated, check it out\n $ cat {}",
                        self.file_name.clone()
                    )
                    .as_str(),
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}
