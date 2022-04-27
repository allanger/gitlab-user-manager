pub(crate) use std::{io::{Error, ErrorKind, Result}, str::FromStr};
pub(crate) enum Versions {
    V1,
}

impl FromStr for Versions {
    fn from_str(input: &str) -> Result<Versions> {
        match input {
            "V1" => Ok(Versions::V1),
            _ => Err(Error::new(
                ErrorKind::NotFound,
                format!("Version {} doesn't exists", input),
            )),
        }
    }
    type Err = Error;
}

pub(crate) trait Version {
    fn get_version(&self) -> Result<Versions>;
}
