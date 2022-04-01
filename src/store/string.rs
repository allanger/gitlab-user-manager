use crate::store::AccessUnit;
use crate::store::Store;

use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

pub(crate) struct StringStore {
    data: String,
}

impl StringStore {
    pub(crate) fn new(data: String) -> Self {
        Self { data }
    }
}

impl Store for StringStore {
    // Get store from the string which is taken from the gum configuration file.
    fn get(&self) -> Result<HashMap<u64, AccessUnit>> {
        let d: std::result::Result<HashMap<u64, AccessUnit>, _> = serde_json::from_str(&self.data);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
    fn write(&self, _: std::collections::HashMap<u64, AccessUnit>) -> Result<()> {
        todo!()
    }
}
