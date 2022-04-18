use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Error, ErrorKind},
};

use crate::output::out_message::OutMessage;

use super::access_level::AccessLevel;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) enum EntityType {
    User,
    Group,
}

impl Default for EntityType {
    fn default() -> Self {
        Self::User
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub(crate) struct AccessUnit {
    pub(crate) entity: EntityType,
    pub(crate) projects: HashMap<u64, AccessLevel>,
    pub(crate) namespaces: HashMap<u64, AccessLevel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub(crate) struct State {
    data: HashMap<u64, AccessUnit>,
}

impl State {
    pub(crate) fn new(data: HashMap<u64, AccessUnit>) -> Self {
        Self { data }
    }

    /// Get a reference to the state's data.
    #[must_use]
    pub(crate) fn data(&self) -> &HashMap<u64, AccessUnit> {
        &self.data
    }
}

impl AccessUnit {
    pub(crate) fn new_simple(entity_type: EntityType) -> Self {
        Self {
            entity: entity_type,
            projects: Default::default(),
            namespaces: Default::default(),
        }
    }
    pub(crate) fn write_to_file(
        state: HashMap<u64, AccessUnit>,
        file_name: String,
    ) -> Result<(), Error> {
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
        match serde_json::to_writer(&f, &state) {
            Ok(()) => return Ok(()),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        }
    }
    pub(crate) fn from_string(data: String) -> Result<HashMap<u64, AccessUnit>, Error> {
        let d: std::result::Result<HashMap<u64, AccessUnit>, _> = serde_json::from_str(&data);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
    pub(crate) fn read_from_file(file_name: String) -> Result<HashMap<u64, AccessUnit>, Error> {
        let f = OpenOptions::new().write(true).read(true).open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        // TODO: Handle serde error
        let d: std::result::Result<HashMap<u64, AccessUnit>, _> = serde_json::from_reader(&f);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}
