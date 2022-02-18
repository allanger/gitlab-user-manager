use core::fmt;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Copy, Clone)]
pub(crate) enum AccessLevel {
    GUEST,
    REPORTER,
    DEVELOPER,
    MAINTAINER,
    OWNER,
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
