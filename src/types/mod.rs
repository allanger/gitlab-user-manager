pub(crate) mod v1;

use std::io::Result;
pub(crate) enum Versions {
    V1,
}
pub(crate) trait Version {
    fn get_version() -> Result<Versions>;
}
