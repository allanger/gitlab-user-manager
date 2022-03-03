pub(crate) mod init;
pub(crate) mod search;
pub(crate) mod sync;
pub(crate) mod teams;
pub(crate) mod users;

use std::io::Result;

pub(crate) trait Cmd<'a> {
    fn exec(&self) -> Result<()>;
}
