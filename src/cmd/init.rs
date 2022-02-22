use std::io::{Error, ErrorKind};

use clap::App;

use crate::{cmd::Cmd, types::config::Config};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_init_cmd() -> App<'static> {
    return App::new("init").about("Create a default yaml file in the current directory");
}

pub(crate) struct InitCmd;

pub(crate) fn prepare<'a>() -> Result<impl Cmd<'a>, Error> {
    Ok(InitCmd)
}

impl<'a> Cmd<'a> for InitCmd {
    fn exec(&self) -> Result<(), Error> {
        let f = "gum-config.yaml";
        println!("Initializing gum config {:?}", f);

        let file = match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(f)
        {
            Ok(file) => file,
            // TODO: Should be more informative
            Err(_error) => {
                return match _error.kind() {
                    ErrorKind::AlreadyExists => {
                        return Err(Error::new(
                            _error.kind(),
                            "config file already exists in specified directory",
                        ))
                    }
                    _ => Err(Error::new(ErrorKind::AlreadyExists, _error)),
                }
            }
        };

        let new_config: Config = Default::default();

        serde_yaml::to_writer(file, &new_config).unwrap();
        Ok(())
    }
}
