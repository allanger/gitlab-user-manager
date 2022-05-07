mod file;
mod string;
use crate::output::out_message::OutMessage;
use crate::types::v1::AccessUnit;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

pub(crate) trait Store {
    fn get(&self) -> Result<HashMap<u64, AccessUnit>>;
    fn write(&self, data: HashMap<u64, AccessUnit>) -> Result<()>;
}

pub(crate) fn get_store_type(source: String) -> Result<Box<dyn Store>> {
    lazy_static! {
        static ref FILE: Regex = Regex::new(r#"^(\.?/.*)$"#).unwrap();
    };
    lazy_static! {
        static ref STRING: Regex = Regex::new(r#"^\{.*\}$"#).unwrap();
    };
    if FILE.is_match(&source) {
        OutMessage::message_info_with_alias("Will try to get a state from the file");
        Ok(Box::new(file::FileStore::new(source)))
    } else if STRING.is_match(&source) {
        OutMessage::message_info_with_alias("Will try to get a state from the config");
        Ok(Box::new(string::StringStore::new(source)))
    } else {
        OutMessage::message_error("Dude, I don't know where to get a state from");
        Err(Error::new(ErrorKind::InvalidData, "unknown store source"))
    }
}
