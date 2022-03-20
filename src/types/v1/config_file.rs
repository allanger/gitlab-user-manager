use std::{
    fs::OpenOptions,
    io::{Error, ErrorKind, Result},
};

use serde::{Deserialize, Serialize};

use super::{config::Config, meta::Meta};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ConfigFile {
    pub(crate) meta: Meta,
    pub(crate) config: Config,
    pub(crate) state: String,
}

impl ConfigFile {
    pub(crate) fn read(file_name: String) -> Result<Self> {
        let f = OpenOptions::new().write(true).read(true).open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        // TODO: Handle serde error
        let d: std::result::Result<ConfigFile, _> = serde_yaml::from_reader(&f);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
    pub(crate) fn write(&self, file_name: String) -> Result<()> {
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };

        match serde_yaml::to_writer(&f, &self) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}
