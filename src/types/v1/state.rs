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
    source: String,
    data: HashMap<u64, AccessUnit>,
}

impl State {
    pub(crate) fn get(source: String) {
        lazy_static! {
            static ref S3: Regex = Regex::new("^s3://(.*)$").unwrap();
        };
        lazy_static! {
            static ref FILE: Regex = Regex::new(r#"^(\.?/.*)$"#).unwrap();
        };
        lazy_static! {
            static ref STRING: Regex = Regex::new(r#"^\{.*\}$"#).unwrap();
        };
        if S3.is_match(&source) {
            OutMessage::message_info_with_alias("Will try to get a state from the s3 bucket");
        } else if FILE.is_match(&source) {
            OutMessage::message_info_with_alias("Will try to get a state from the file");
        } else if STRING.is_match(&source) {
            OutMessage::message_info_with_alias("Will try to get a state from the config");
        } else {
            OutMessage::message_error("Dude, I don't know where to get a state from");
        }
    }

    pub(crate) fn from_string(source: String) {}
}

impl AccessUnit {
    pub(crate) fn get(source: String) {
        lazy_static! {
            static ref S3: Regex = Regex::new("S3://.*").unwrap();
        };
        match source {
            _ => todo!(),
        }
    }

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
