use core::fmt;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Debug, Serialize, Deserialize, Copy, Clone, EnumIter)]
pub(crate) enum AccessLevel {
    Guest,
    Reporter,
    Developer,
    Maintainer,
    Owner,
    Admin,
}
impl FromStr for AccessLevel {
    fn from_str(input: &str) -> Result<AccessLevel, Error> {
        match input {
            "guest" => Ok(AccessLevel::Guest),
            "reporter" => Ok(AccessLevel::Reporter),
            "developer" => Ok(AccessLevel::Developer),
            "maintainer" => Ok(AccessLevel::Maintainer),
            "owner" => Ok(AccessLevel::Owner),
            "admin" => Ok(AccessLevel::Admin),
            _ => Err(Error::new(
                ErrorKind::NotFound,
                format!("access level {} can not be found", input),
            )),
        }
    }
    type Err = Error;
}

impl AccessLevel {
    pub(crate) fn to_gitlab_access_level(self) -> gitlab::api::common::AccessLevel {
        match self {
            AccessLevel::Guest => gitlab::api::common::AccessLevel::Guest,
            AccessLevel::Reporter => gitlab::api::common::AccessLevel::Reporter,
            AccessLevel::Developer => gitlab::api::common::AccessLevel::Developer,
            AccessLevel::Maintainer => gitlab::api::common::AccessLevel::Maintainer,
            AccessLevel::Owner => gitlab::api::common::AccessLevel::Owner,
            AccessLevel::Admin => gitlab::api::common::AccessLevel::Admin,
        }
    }
    pub(crate) fn from_gitlab_access_level(access_level: gitlab::AccessLevel) -> Self {
        match access_level {
            gitlab::AccessLevel::Guest => AccessLevel::Guest,
            gitlab::AccessLevel::Reporter => AccessLevel::Reporter,
            gitlab::AccessLevel::Developer => AccessLevel::Developer,
            gitlab::AccessLevel::Maintainer => AccessLevel::Maintainer,
            gitlab::AccessLevel::Owner => AccessLevel::Owner,
            gitlab::AccessLevel::Admin => AccessLevel::Admin,
            gitlab::AccessLevel::Anonymous => todo!(),
        }
    }
}

impl fmt::Display for AccessLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::Guest
    }
}
