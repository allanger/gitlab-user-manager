use std::io::Error;
use std::result::Result;

use clap::ArgMatches;

use crate::{pkg::config, types::types};

pub fn teams_pkg(sub_matches: &ArgMatches) -> Result<(), Error> {
    match sub_matches.subcommand() {
        Some(("create", sub_matches)) => create(
            sub_matches
                .value_of("TEAM_NAME")
                .expect("Team name is required"),
        ),
        _ => unreachable!(),
    }
    Ok(())
}

fn create(team_name: &str) -> () {
    let mut config = match config::read_config() {
        Ok(c) => c,
        Err(_error) => return (),
    };

    let new_team = types::Team {
        name: team_name.to_string(),
        projects: None,
    };

    if config.teams.as_mut().unwrap().iter().any(|i| i.name == new_team.name) {
      println!("NO WAAAAY");
      return ();
    }
    config.teams.as_mut().unwrap().extend([new_team]);
    println!("Read YAML string: {:?}", config);
    let _ = match config::write_config(config) {
        Ok(()) => return (),
        Err(_error) => return (),
    };
}
