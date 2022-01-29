use std::process::exit;

use crate::types::types::{Config, Team};

pub fn init_pkg() -> () {
    let file_name = "gum-config.yaml";
    println!("Initializing gum config {:?}", file_name);

    // Creating a config file
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name);

    let f = match f {
        Ok(file) => file,
        Err(_error) => {
            println!("File not found: { }", file_name);
            exit(1);
        }
    };

    // Create default empty config
    let new_config = Config {
        teams: Some(vec![Team {
            name: "default".to_string(),
            projects: None,
        }]),
        users: None,
    };
    // Write to file
    serde_yaml::to_writer(f, &new_config).unwrap();
}
