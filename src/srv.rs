pub(crate) mod srv {
    use crate::types::types::{Config, Team};
    use std::io::Error;

    pub fn new_srv() -> impl Init {
        Cmd
    }

    struct Cmd;
    pub trait Init {
        fn exec(&self, f: String) -> Result<(), Error>;
    }

    impl Init for Cmd {
        fn exec(&self, f: String) -> Result<(), Error> {
            println!("Initializing gum config {:?}", f);

            let file = match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(f)
            {
                Ok(file) => file,
                // TODO: Should be more informative
                Err(_error) => {
                    return match _error.kind() {
                        std::io::ErrorKind::AlreadyExists => {
                            return Err(Error::new(
                                _error.kind(),
                                "config file already exists in specified directory",
                            ))
                        }
                        _ => Err(Error::new(std::io::ErrorKind::AlreadyExists, _error)),
                    }
                }
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
        use std::fs::File;

        use super::{new_srv, Init};

        #[test]
        fn it_works() {
            let tempdir = tempfile::tempdir().unwrap();
            let filename = tempdir.path().join("gum-config.yaml");
            assert!(new_srv()
                .exec(filename.into_os_string().into_string().unwrap())
                .is_ok());
        }
        #[test]

        fn it_doesnt_work() {
            let tempdir = tempfile::tempdir().unwrap();
            let filename = tempdir.path().join("gum-config.yaml");
            let _ = File::create(&filename);
            let result = new_srv().exec(filename.into_os_string().into_string().unwrap());
            assert!(result.is_err());
            let actual_inner_error_disp = format!("{}", result.unwrap_err().into_inner().unwrap());
            let expected_inner_error_disp =
                format!("config file already exists in specified directory");
            assert_eq!(actual_inner_error_disp, expected_inner_error_disp);
        }
    }
}
