use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Result};
use std::{io::Error, str::FromStr};

use super::common::{Version, Versions};

// The first version of a config file
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ConfigFile {
    meta: Meta,
    config: Config,
    state: String,
}

impl ConfigFile {
    /// Get a reference to the config file's meta.
    #[must_use]
    pub(crate) fn meta(&self) -> &Meta {
        &self.meta
    }

    /// Get a reference to the config file's config.
    #[must_use]
    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    /// Get a mutable reference to the config file's config.
    #[must_use]
    pub(crate) fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Get a reference to the config file's state.
    #[must_use]
    pub(crate) fn state(&self) -> &str {
        self.state.as_ref()
    }

    /// Set the config file's state.
    pub(crate) fn set_state(&mut self, state: String) {
        self.state = state;
    }

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

impl Version for ConfigFile {
    fn get_version(&self) -> Result<Versions> {
        Versions::from_str(&self.meta().version())
    }
}

// First version of metadata object in the config file
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct Meta {
    version: String,
}

impl Meta {
    /// Get a reference to the meta's version.
    #[must_use]
    pub(crate) fn version(&self) -> &str {
        self.version.as_ref()
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
        }
    }
}

// First Version of a config object
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub(crate) teams: Vec<Team>,
    #[serde(default)]
    pub(crate) users: Vec<User>,
    #[serde(default)]
    pub(crate) groups: Vec<Group>,
}

impl Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            teams: vec![Team {
                name: "default".to_string(),
                ..Default::default()
            }],
            users: Default::default(),
            groups: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Group {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) projects: Vec<Project>,
    pub(crate) namespaces: Vec<Namespace>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Namespace {
    pub(crate) name: String,
    pub(crate) access_level: AccessLevel,
    pub(crate) id: u64,
    pub(crate) url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) id: u64,
    pub(crate) access_level: AccessLevel,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct Team {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) projects: Vec<Project>,
    #[serde(default)]
    pub(crate) namespaces: Vec<Namespace>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) teams: Vec<String>,
    pub(crate) projects: Vec<Project>,
    pub(crate) namespaces: Vec<Namespace>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Copy, Clone)]
pub(crate) enum AccessLevel {
    Guest,
    Reporter,
    Developer,
    Maintainer,
    Owner,
    Admin,
}

impl FromStr for AccessLevel {
    fn from_str(input: &str) -> Result<AccessLevel> {
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

    /// Get a mutable reference to the state's data.
    #[must_use]
    pub(crate) fn data_mut(&mut self) -> &mut HashMap<u64, AccessUnit> {
        &mut self.data
    }

    /// Set the state's data.
    pub(crate) fn set_data(&mut self, data: HashMap<u64, AccessUnit>) {
        self.data = data;
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
    pub(crate) fn write_to_file(state: HashMap<u64, AccessUnit>, file_name: String) -> Result<()> {
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
    pub(crate) fn from_string(data: String) -> Result<HashMap<u64, AccessUnit>> {
        let d: std::result::Result<HashMap<u64, AccessUnit>, _> = serde_json::from_str(&data);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
    pub(crate) fn read_from_file(file_name: String) -> Result<HashMap<u64, AccessUnit>> {
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
