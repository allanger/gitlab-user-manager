pub(crate) mod srv {
    use std::io::Error;

    pub fn new_srv() -> impl SrvActions {
        Srv
    }

    struct Srv;
    pub trait SrvActions {
        fn init(&self) -> Result<(), Error>;
    }

    impl SrvActions for Srv {
        fn init(&self) -> Result<(), Error> {
            let file = |f: String| {
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(f)
            };
            init_mod::init(file)
        }
    }

    mod init_mod {
        use std::{
            fs::File,
            io::{Error, ErrorKind},
            result::Result,
        };

        use crate::types::types::{Config, Team};
        pub fn init<F>(mut f: F) -> Result<(), Error>
        where
            F: FnMut(String) -> Result<File, std::io::Error>,
        {
            //  TODO: Add a possibility to use other file names
            let file_name = "gum-config.yam l";
            println!("Initializing gum config {:?}", file_name);

            let file = match f(file_name.to_string()) {
                Ok(file) => file,
                Err(_error) => return Err(Error::new(ErrorKind::AlreadyExists, _error)),
            };

            let new_config = Config {
                teams: Some(vec![Team {
                    name: "default".to_string(),
                    projects: None,
                }]),
                users: None,
            };

            serde_yaml::to_writer(file, &new_config).unwrap();
            Ok(())
        }
    }
    #[cfg(test)]
    mod tests {
        use std::fs;

        use super::init_mod;

        #[test]
        fn create_file() {
            let file = |f: String| {
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open("/tmp/gum")
            };
            assert!(init_mod::init(file).is_ok());
            fs::remove_file("/tmp/gum");
        }
        #[test]
        fn create_file_twice() {
            let data = "Some data!";
            fs::write("/tmp/gum2", data).expect("Unable to write file");
            
            let file = |f: String| {
                std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open("/tmp/gum2")
            };
            assert!(init_mod::init(file).is_err(), "fuck you");
            fs::remove_file("/tmp/gum2");

        }
    }
}
