use crate::types::v1::config_file::ConfigFile;
use std::io::Result;

#[derive(Default)]
pub(crate) struct InitService {
    config_file: ConfigFile,
}

impl InitService {
    pub(crate) fn new() -> Self {
        Self {
            config_file: Default::default(),
        }
    }

    pub(crate) fn parse_groups(groups: Vec<u64>) -> Result<ConfigFile> {
        Ok(ConfigFile::default())
    }

    pub(crate) fn create_file() -> Result<()> {
        Ok(())
    }
}
