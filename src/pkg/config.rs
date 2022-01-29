use std::io::Error;
use std::io::ErrorKind;
// use std::io::Result;
use std::result::Result;

pub fn read_config() -> Result<crate::types::types::Config, Error> {
    let file_name = "gum-config.yaml";

    let f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            return Err(_error);
        }
    };

    let d: Result<crate::types::types::Config, _> = serde_yaml::from_reader(&f);

    let _ = match d {
        Ok(r) => return Ok(r),
        Err(_error) => {
            return Err(Error::new(ErrorKind::Other, _error.to_string()));
        }
    };
}

pub fn write_config(config: crate::types::types::Config) -> Result<(), Error> {
    let file_name = "gum-config.yaml";

    let f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
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
