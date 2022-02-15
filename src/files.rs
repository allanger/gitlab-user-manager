use std::io::Error;
use std::io::ErrorKind;
use std::result::Result;

pub(crate) fn read_config() -> Result<crate::types::config::Config, Error> {
    let file_name = "gum-config.yaml";

    let f = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(file_name);
    // TODO: Handle different reader errors
    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let d: Result<crate::types::config::Config, _> = serde_yaml::from_reader(&f);

    let _ = match d {
        Ok(r) => return Ok(r),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn write_config(config: crate::types::config::Config) -> Result<(), Error> {
    let file_name = "gum-config.yaml";

    let f = std::fs::OpenOptions::new()
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

pub(crate) fn read_state() -> Result<crate::types::state::State, Error> {
    let file_name = "gum-state.yaml";

    let f = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(file_name);
    // TODO: Handle different reader errors
    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let d: Result<crate::types::state::State, _> = serde_yaml::from_reader(&f);

    let _ = match d {
        Ok(r) => return Ok(r),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub(crate) fn write_state(state: crate::types::state::State) -> Result<(), Error> {
    let file_name = "gum-state.yaml";

    let f = std::fs::OpenOptions::new()
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
