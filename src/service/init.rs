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

    pub(crate) fn parse_groups(&mut self, groups: Vec<u64>) -> &Self {
        if !groups.is_empty() {
            return self;
        }
        self
    }

    pub(crate) fn save(&self, file_name: String) -> Result<()> {
        // File::save()
        Ok(())
    }

    /// Set the init service's config file.
    pub(crate) fn set_config_file(&mut self, config_file: ConfigFile) {
        self.config_file = config_file;
    }
}
