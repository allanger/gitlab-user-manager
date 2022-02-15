use core::fmt;

use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) access_level: AccessLevel,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: u64::MIN,
            access_level: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Copy, Clone)]
pub(crate) enum AccessLevel {
    GUEST,
    REPORTER,
    DEVELOPER,
    MAINTAINER,
}

impl FromStr for AccessLevel {
    fn from_str(input: &str) -> Result<AccessLevel, Error> {
        match input {
            "guest" => Ok(AccessLevel::GUEST),
            "reporter" => Ok(AccessLevel::REPORTER),
            "developer" => Ok(AccessLevel::DEVELOPER),
            "maintainer" => Ok(AccessLevel::MAINTAINER),
            _ => Err(Error::new(
                ErrorKind::NotFound,
                format!("access level {} can not be found", input),
            )),
        }
    }
    type Err = Error;
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::GUEST
    }
}
