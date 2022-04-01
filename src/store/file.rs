use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Error, ErrorKind, Result},
};

use super::Store;
use crate::types::v1::state::AccessUnit;

pub(crate) struct FileStore {
    file_path: String,
}

impl FileStore {
    pub(crate) fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl Store for FileStore {
    fn get(&self) -> Result<HashMap<u64, AccessUnit>> {
        let f = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&self.file_path);
        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        // TODO: Handle serde error
        let d: std::result::Result<HashMap<u64, AccessUnit>, _> = serde_json::from_reader(&f);
        match d {
            Ok(r) => Ok(r),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }

    fn write(&self, data: HashMap<u64, AccessUnit>) -> Result<()> {
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.file_path);
        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        match serde_json::to_writer(&f, &data) {
            Ok(()) => return Ok(()),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    use crate::store::Store;
    use crate::types::v1::access_level::AccessLevel;
    use crate::types::v1::state::AccessUnit;
    use crate::types::v1::state::EntityType::User;

    use super::FileStore;

    #[test]
    fn read_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("my-temporary-note.txt");
        let mut file = File::create(file_path.clone()).unwrap();
        let data = r#"{"1":{"entity":"User","projects":{"1":"Developer"},"namespaces":{"1":"Maintainer"}}}"#;

        writeln!(file, "{}", data).unwrap();
        let file_store = FileStore::new(file_path.to_string_lossy().to_string());
        let mut data: HashMap<u64, AccessUnit> = HashMap::new();
        let mut projects: HashMap<u64, AccessLevel> = HashMap::new();
        let mut namespaces: HashMap<u64, AccessLevel> = HashMap::new();
        projects.insert(1, AccessLevel::Developer);
        namespaces.insert(1, AccessLevel::Maintainer);
        let access_unit = AccessUnit {
            entity: User,
            projects,
            namespaces,
        };
        data.insert(1, access_unit);
        assert_eq!(data, file_store.get().unwrap());
    }
}
