use std::{collections::HashMap, io::{Error, ErrorKind}, fs::OpenOptions};

use serde::{Deserialize, Serialize};

use super::access_level::AccessLevel;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub(crate) struct State {
    pub(crate) projects: HashMap<u64, AccessLevel>,
    pub(crate) groups: HashMap<u64, AccessLevel>,
}

impl State {
    pub(crate) fn write_to_file(
        state: HashMap<u64, State>,
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

        let _ = match serde_json::to_writer(&f, &state) {
            Ok(()) => return Ok(()),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        };
    }
    pub(crate) fn read_from_file(file_name: String) -> Result<HashMap<u64, State>, Error> {
        let f = OpenOptions::new().write(true).read(true).open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        // TODO: Handle serde error
        let d: std::result::Result<HashMap<u64, State>, _> = serde_json::from_reader(&f);
        match d {
            Ok(r) => return Ok(r),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        };
    }
}