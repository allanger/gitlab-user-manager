mod file;
mod string;
use crate::types::v1::state::AccessUnit;
use std::collections::HashMap;
use std::io::Result;

struct StoreUnit;

pub(crate) trait Store {
    fn get(&self) -> Result<HashMap<u64, AccessUnit>>;
    fn write(&self, data: HashMap<u64, AccessUnit>) -> Result<()>;
}
