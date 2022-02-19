use std::fs::OpenOptions;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::result::Result;

use uuid::Uuid;

use crate::types::config::Config;
use crate::types::state::State;

pub(crate) fn read_config() -> Result<Config, Error> {
    let file_name = "gum-config.yaml";

    let f = OpenOptions::new().write(true).read(true).open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };
    let d: Result<Config, _> = serde_yaml::from_reader(&f);
    // return d
    let _ = match d {
        Ok(r) => return Ok(r),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn write_config(config: Config) -> Result<(), Error> {
    let file_name = "gum-config.yaml";

    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let _ = match serde_yaml::to_writer(&f, &config) {
        Ok(()) => return Ok(()),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn read_state() -> Result<Vec<State>, Error> {
    let file_name = "gum-state.yaml";

    let f = OpenOptions::new().write(true).read(true).open(file_name);
    // TODO: Handle different reader errors
    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let d: Result<Vec<State>, _> = serde_yaml::from_reader(&f);

    let _ = match d {
        Ok(r) => return Ok(r),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn write_state(state: Vec<State>, dry: bool) -> Result<(), Error> {
    let file_name;
    if dry {
        let file_uuid = Uuid::new_v4();
        file_name = format!("/tmp/gum-state-{}.yaml", file_uuid);
    } else {
        file_name = "gum-state.yaml".to_string();
    }

    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(true)
        .open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let _ = match serde_yaml::to_writer(&f, &state) {
        Ok(()) => return Ok(()),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn state_exists() -> bool {
    let file_name = "gum-state.yaml";
    Path::new(file_name).exists()
}
