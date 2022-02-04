use crate::types::types::{Config, Team};
use std::io::Error;

pub fn init_srv() -> Option<Error> {
    //  TODO: Add possibility use other file names
    let file_name = "gum-config.yaml";
    println!("Initializing gum config {:?}", file_name);
    
    // Has to be mockable
    if std::path::Path::new(file_name).exists() {
        return Some(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "config file already exists in the current dir",
        ));
    }
    
    // Creating a config file
    // Has to be mockable
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name);   

    let f = match f {
        Ok(file) => file,
        Err(_error) => return Some(_error),
    };

    // Create default empty config
    // Has to be mockable
    let new_config = Config {
        teams: Some(vec![Team {
            name: "default".to_string(),
            projects: None,
        }]),
        users: None,
    };
    // Write to file
    serde_yaml::to_writer(f, &new_config).unwrap();
    None
}
